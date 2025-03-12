use lapin::{options::*, BasicProperties, Connection, ConnectionProperties};
use tracing::info;
pub async fn publish_to_queue(short_code: &str, original_url: &str) -> Result<(), lapin::Error> {
    info!("Publishing to RabbitMQ: {}:{}", short_code, original_url);
    let rabbitmq_url = std::env::var("RABBITMQ_URL").expect("RABBITMQ_URL must be set");
    info!("RabbitMQ URL: {}", rabbitmq_url);
    let conn = Connection::connect(&rabbitmq_url, ConnectionProperties::default()).await?;
    info!("Connected to RabbitMQ");
    let channel = conn.create_channel().await?;
    info!("Channel created");
    let payload = format!("{}:{}", short_code, original_url);
    channel
        .basic_publish("", "url_queue", BasicPublishOptions::default(), payload.as_bytes(), BasicProperties::default())
        .await?;
    
    Ok(())
}
