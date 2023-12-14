use crate::replay::ReplayCommand;
use anyhow::{Context, Result};
use redis::aio::ConnectionManager;
use redis::IntoConnectionInfo;
use std::fmt::Display;

pub struct RedisManager {
    connection: ConnectionManager,
}

impl RedisManager {
    pub async fn connect<T: IntoConnectionInfo + Display + Clone>(addr: T) -> Result<Self> {
        let addr_clone = addr.clone();
        let client = redis::Client::open(addr)
            .with_context(|| format!("Failed to create redis client for {}", addr_clone))?;
        let connection = client
            .get_connection_manager_with_backoff(2, 100, 6)
            .await
            .with_context(|| format!("Failed to connect to redis {}", addr_clone))?;
        Ok(Self { connection })
    }

    pub async fn apply(&mut self, command: ReplayCommand) -> Result<()> {
        let command: redis::Cmd = command
            .try_into()
            .context("Failed to map command from file to redis command")?;
        let _ = command
            .query_async(&mut self.connection)
            .await
            .context("Failed to query redis")?;
        Ok(())
    }
}
