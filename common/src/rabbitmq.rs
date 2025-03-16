use lapin::{Connection, ConnectionProperties};
use tokio::time::{sleep, timeout, Duration};
use tracing::info;

/// Connects to RabbitMQ with retry logic and timeout handling.
pub async fn connect_to_rabbitmq(url: &str) -> Connection {
    let max_retries = 10;
    let base_delay = Duration::from_millis(500);
    let max_delay = Duration::from_secs(5);
    let total_timeout = Duration::from_secs(15);

    let mut attempts = 0;

    let connection = timeout(total_timeout, async {
        loop {
            match Connection::connect(url, ConnectionProperties::default()).await {
                Ok(conn) => {
                    info!("✅ Connected to RabbitMQ!");
                    return Ok(conn);
                }
                Err(e) => {
                    attempts += 1;
                    tracing::error!("🚨 Failed to connect to RabbitMQ (attempt {}/{}): {}", attempts, max_retries, e);
                    if attempts >= max_retries {
                        return Err("❌ Exceeded max retries. RabbitMQ is unavailable.");
                    }
                    
                    let delay = std::cmp::min(base_delay * (2_u32.pow(attempts) as u32), max_delay);
                    sleep(delay).await;
                }
            }
        }
    })
    .await;

    connection.expect("❌ Timed out waiting for RabbitMQ").expect("❌ Failed to connect to RabbitMQ")
}
