use crate::ConnectionError;
use crate::client::{RedisClient, RedisConfig};
use crate::error::Result;
use redis::aio::ConnectionManager;
use std::time::Duration;

/// Redis 连接池配置
#[derive(Debug, Clone)]
pub struct PoolConfig {
    /// 连接超时时间
    pub connection_timeout: Duration,
    /// 重连间隔
    pub retry_interval: Duration,
    /// 最大重连次数
    pub max_retries: u32,
    /// 保持连接活跃
    pub keep_alive: bool,
}

impl Default for PoolConfig {
    fn default() -> Self {
        Self {
            connection_timeout: Duration::from_secs(30),
            retry_interval: Duration::from_millis(100),
            max_retries: 3,
            keep_alive: true,
        }
    }
}

/// Redis 连接池 - 使用 redis connection-manager
pub struct RedisPool;

impl RedisPool {
    /// 创建新的 Redis 连接管理器
    ///
    /// # Arguments
    ///
    /// * `config` - Redis 配置信息
    /// * `_pool_config` - 连接池配置（暂时保留用于兼容性）
    ///
    /// # Returns
    ///
    /// 返回 ConnectionManager 实例或错误
    pub async fn create(config: RedisConfig) -> Result<ConnectionManager> {
        // 构建 Redis URL
        let redis_url = RedisClient::build_redis_url(&config)?;

        // 创建 Redis 客户端
        let client = redis::Client::open(redis_url)
            .map_err(|e| ConnectionError::PoolCreation(format!("Failed to create client: {e}")))?;

        // 创建连接管理器
        let manager = ConnectionManager::new(client).await.map_err(|e| {
            ConnectionError::PoolCreation(format!("Failed to create connection manager: {e}"))
        })?;

        Ok(manager)
    }
}
