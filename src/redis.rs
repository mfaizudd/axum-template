use deadpool_redis::redis::{self, AsyncCommands};
use serde::{Deserialize, Serialize};

pub type RedisPool = deadpool_redis::Pool;

pub struct CommandBuilder<'a> {
    pool: &'a RedisPool,
    key: &'a str,
}

pub fn command<'a>(pool: &'a RedisPool, key: &'a str) -> CommandBuilder<'a> {
    CommandBuilder { pool, key }
}

impl<'a> CommandBuilder<'a> {
    pub async fn get<T: for<'de> Deserialize<'de>>(&self) -> Result<Option<T>, anyhow::Error> {
        let mut client = self.pool.get().await?;
        let value: Option<String> = redis::cmd("GET")
            .arg(self.key)
            .query_async(&mut client)
            .await?;
        Ok(value.map(|v| serde_json::from_str(&v)).transpose()?)
    }

    pub async fn set<T: Serialize>(self, value: &T) -> Result<CommandBuilder<'a>, anyhow::Error> {
        let mut client = self.pool.get().await?;
        let value = serde_json::to_string(&value)?;
        client.set(self.key, &value).await?;
        Ok(self)
    }

    pub async fn expire(self, ttl: chrono::Duration) -> Result<CommandBuilder<'a>, anyhow::Error> {
        let mut client = self.pool.get().await?;
        let ttl = usize::try_from(ttl.num_seconds())?;
        client.expire(self.key, ttl).await?;
        Ok(self)
    }

    pub async fn delete(self) -> Result<(), anyhow::Error> {
        let mut client = self.pool.get().await?;
        client.del(self.key).await?;
        Ok(())
    }
}
