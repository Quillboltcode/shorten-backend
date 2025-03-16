use lapin::{options::*, types::FieldTable, BasicProperties};
use tracing::info;
use common::rabbitmq::connect_to_rabbitmq;


pub async fn publish_to_queue(short_code: &str, original_url: &str) -> Result<(), lapin::Error> {
    info!("📢 Publishing to RabbitMQ: {}:{}", short_code, original_url);

    let rabbitmq_url = std::env::var("RABBITMQ_URL").expect("RABBITMQ_URL must be set");
    info!("🐇 RabbitMQ URL: {}", rabbitmq_url);

    // Use `connect_to_rabbitmq` instead of direct connection
    let conn = connect_to_rabbitmq(&rabbitmq_url).await;
    let channel = conn.create_channel().await?;
    info!("📡 Channel created");

    let payload = format!("{}:{}", short_code, original_url);
    
    channel
        .basic_publish(
            "", 
            "url_queue", 
            BasicPublishOptions::default(), 
            payload.as_bytes(), 
            BasicProperties::default(),
        )
        .await?;
    
    info!("✅ Message successfully published to queue");
    Ok(())
}
