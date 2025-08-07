use crate::client::{RedisClient, RedisConfig};
use crate::error::Result;
use crate::pool::RedisPool;
use redis::aio::ConnectionManager;

/// Redis 连接器 - 统一的入口点，负责创建客户端和连接管理器
#[derive(Debug, Clone)]
pub struct RedisConnector {
    /// Redis 主机地址
    pub host: String,
    /// Redis 端口
    pub port: u16,
    /// Redis 密码
    pub password: String,
    /// Redis 数据库
    pub db: u8,
}

impl Default for RedisConnector {
    fn default() -> Self {
        Self {
            host: "localhost".to_string(),
            port: 6379,
            password: "".to_string(),
            db: 0,
        }
    }
}

impl RedisConnector {
    /// 创建新的 Redis 连接器
    ///
    /// # Returns
    ///
    /// 返回 RedisConnector 实例
    pub fn new() -> Self {
        Self::default()
    }

    /// 创建单个 Redis 客户端
    ///
    /// # Returns
    ///
    /// 返回 RedisClient 实例或错误
    pub fn client(&self) -> Result<redis::Client> {
        RedisClient::create(RedisConfig {
            host: self.host.clone(),
            port: self.port,
            password: self.password.clone(),
            db: self.db,
        })
    }

    /// 创建 Redis 连接管理器
    ///
    /// # Returns
    ///
    /// 返回 ConnectionManager 实例或错误
    pub async fn connection_manager(&self) -> Result<ConnectionManager> {
        let redis_config = RedisConfig {
            host: self.host.clone(),
            port: self.port,
            password: self.password.clone(),
            db: self.db,
        };

        RedisPool::create(redis_config).await
    }

    /// 设置主机
    pub fn host(mut self, host: String) -> Self {
        self.host = host;
        self
    }

    /// 设置端口
    pub fn port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    /// 设置密码
    pub fn password(mut self, password: String) -> Self {
        self.password = password;
        self
    }

    /// 设置数据库
    pub fn db(mut self, db: u8) -> Self {
        self.db = db;
        self
    }
}
