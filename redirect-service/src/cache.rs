use redis::{AsyncCommands, Client, RedisResult};

pub struct RedisCache {
    client: Client,
}

impl RedisCache {
    /// Initialize a new Redis connection
    pub async fn new(redis_url: &str) -> RedisResult<Self> {
        let client = Client::open(redis_url)?;
        Ok(Self { client })
    }

    /// Fetch a URL from Redis cache
    pub async fn get(&self, short_code: &str) -> Option<String> {
        let mut conn = self.client.get_multiplexed_async_connection().await.ok()?;
        match conn.get(short_code).await {
            Ok(url) => Some(url),
            Err(_) => None,
        }
    }

    /// Store a URL in Redis cache with an expiry time
    pub async fn set(&self, short_code: &str, original_url: &str, ttl: usize) -> RedisResult<()> {
        let mut conn = self.client.get_multiplexed_async_connection().await?;
        conn.set_ex(short_code, original_url, ttl.try_into().unwrap()).await
    }
}
