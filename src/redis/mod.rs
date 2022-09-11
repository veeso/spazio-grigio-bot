//! # Redis
//!
//! Redis client module

use redis::{AsyncCommands, Client, FromRedisValue, RedisError, ToRedisArgs};

pub type RedisResult<T> = Result<T, RedisError>;

pub struct RedisClient {
    client: Client,
}

impl RedisClient {
    /// Connect to redis
    pub fn connect(url: &str) -> RedisResult<Self> {
        let client = Client::open(url)?;
        debug!("connected to {}", url);
        Ok(Self { client })
    }

    /// Set key
    pub async fn set<V>(&mut self, key: &str, value: V) -> RedisResult<()>
    where
        V: ToRedisArgs + Send + Sync + std::fmt::Debug,
    {
        let mut connection = self.client.get_async_connection().await?;
        debug!("SET {} to {:?}", key, value);
        connection.set(key, value).await
    }

    /// Get key
    pub async fn get<V>(&mut self, key: &str) -> RedisResult<Option<V>>
    where
        V: FromRedisValue,
    {
        let mut connection = self.client.get_async_connection().await?;
        debug!("GET {}", key);
        connection.get(key).await
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use pretty_assertions::assert_eq;

    #[tokio::test]
    async fn should_set_key() {
        let mut client = RedisClient::connect("redis://localhost/").unwrap();
        assert!(client.set("test:key1", "1").await.is_ok());
    }

    #[tokio::test]
    async fn should_get_key() {
        let mut client = RedisClient::connect("redis://localhost/").unwrap();
        assert!(client.set("test:key2", "3").await.is_ok());
        assert_eq!(
            client.get("test:key2").await.unwrap(),
            Some(String::from("3"))
        );
    }

    #[tokio::test]
    async fn should_get_none() {
        let mut client = RedisClient::connect("redis://localhost/").unwrap();
        assert_eq!(client.get::<String>("test:key3").await.unwrap(), None);
    }
}
