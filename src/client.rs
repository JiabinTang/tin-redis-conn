use crate::error::{ConnectionError, Result};

pub struct RedisConfig {
    /// Redis 主机地址
    pub host: String,
    /// Redis 端口
    pub port: u16,
    /// Redis 密码
    pub password: String,
    /// Redis 数据库
    pub db: u8,
}

impl Default for RedisConfig {
    fn default() -> Self {
        Self {
            host: "localhost".to_string(),
            port: 6379,
            password: "".to_string(),
            db: 0,
        }
    }
}

/// Redis 客户端封装
pub struct RedisClient;

impl RedisClient {
    /// 创建新的 Redis 客户端
    ///
    /// # Arguments
    ///
    /// * `config` - Redis 配置信息
    ///
    /// # Returns
    ///
    /// 返回 Redis Client 实例或错误
    pub fn create(config: RedisConfig) -> Result<redis::Client> {
        let redis_url = Self::build_redis_url(&config)?;

        log::debug!("Redis URL: {redis_url}",);

        let client = redis::Client::open(redis_url)?;

        Ok(client)
    }

    /// 构建 Redis URL
    pub fn build_redis_url(config: &RedisConfig) -> Result<String> {
        if config.host.is_empty() {
            return Err(ConnectionError::Configuration(
                "Redis host cannot be empty".to_string(),
            ));
        }

        let redis_url = if config.password.is_empty() {
            format!(
                "redis://{host}:{port}/{db}",
                host = config.host,
                port = config.port,
                db = config.db
            )
        } else {
            format!(
                "redis://:{password}@{host}:{port}/{db}",
                password = config.password,
                host = config.host,
                port = config.port,
                db = config.db
            )
        };

        Ok(redis_url)
    }
}
