use std::env;
use lapin::{options::*, types::FieldTable, Consumer};
use redis::AsyncCommands;
use futures_util::StreamExt;
use common::rabbitmq::connect_to_rabbitmq;
use tracing::info;

pub async fn listen_for_updates(redis: redis::Client) {
    let rabbitmq_url = env::var("RABBITMQ_URL").expect("RABBITMQ_URL must be set");
    
    let conn= connect_to_rabbitmq(&rabbitmq_url).await;
    
    let channel = conn.create_channel().await.expect("Failed to create channel");
    // ✅ Declare the queue before publishing
    channel
            .queue_declare(
                "url_queue",
                QueueDeclareOptions {
                    passive: false,      // The queue must exist
                    durable: true,       // The queue will persist across RabbitMQ restarts
                    auto_delete: false,  // The queue is not deleted when no consumers exist
                    exclusive: false,    // Allow multiple consumers
                    nowait: false,
                },
                FieldTable::default(),
            )
            .await.unwrap();
    info!("✅ Queue declared: url_queue");

    let consumer: Consumer = channel
        .basic_consume("url_queue", "consumer", BasicConsumeOptions::default(), FieldTable::default())
        .await
        .expect("Failed to create consumer");

    println!("Listening for messages on RabbitMQ queue: url_queue");

    tokio::spawn(async move {
        let mut conn = redis.get_multiplexed_async_connection().await.expect("Failed to connect to Redis");

        let mut consumer = consumer;
        while let Some(delivery) = consumer.next().await {
            if let Ok(delivery) = delivery {
                if let Ok(msg) = String::from_utf8(delivery.data.clone()) {
                    let parts: Vec<&str> = msg.split(':').collect();
                    if parts.len() == 2 {
                        let short_code = parts[0];
                        let original_url = parts[1];

                        if let Err(e) = conn.set_ex::<&str, &str, ()>(short_code, original_url, 3600).await {
                            eprintln!("Failed to cache URL in Redis: {}", e);
                        } else {
                            println!("Cached short_code {} -> {}", short_code, original_url);
                        }
                    }
                }

                if let Err(e) = delivery.ack(BasicAckOptions::default()).await {
                    eprintln!("Failed to acknowledge message: {}", e);
                }
            }
        }
    });
}
