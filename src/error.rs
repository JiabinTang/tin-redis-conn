use std::fmt;

/// Redis 连接相关的错误类型
#[derive(Debug)]
pub enum ConnectionError {
    /// Redis 客户端创建失败
    ClientCreation(redis::RedisError),
    /// 连接池创建失败
    PoolCreation(String),
    /// 连接获取失败
    ConnectionAcquisition(redis::RedisError),
    /// 连接管理器错误
    ConnectionManager(redis::RedisError),
    /// 配置错误
    Configuration(String),
    /// 连接超时
    Timeout,
    /// 网络错误
    Network(String),
    /// 序列化错误
    Serialization(String),
    /// 反序列化错误
    Deserialization(String),
}

impl fmt::Display for ConnectionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConnectionError::ClientCreation(e) => write!(f, "Failed to create Redis client: {e}"),
            ConnectionError::PoolCreation(msg) => {
                write!(f, "Failed to create connection pool: {msg}")
            }
            ConnectionError::ConnectionAcquisition(e) => {
                write!(f, "Failed to acquire connection: {e}")
            }
            ConnectionError::ConnectionManager(e) => write!(f, "Connection manager error: {e}"),
            ConnectionError::Configuration(msg) => write!(f, "Configuration error: {msg}"),
            ConnectionError::Timeout => write!(f, "Connection timeout"),
            ConnectionError::Network(msg) => write!(f, "Network error: {msg}"),
            ConnectionError::Serialization(msg) => write!(f, "Serialization error: {msg}"),
            ConnectionError::Deserialization(msg) => write!(f, "Deserialization error: {msg}"),
        }
    }
}

impl std::error::Error for ConnectionError {}

impl From<redis::RedisError> for ConnectionError {
    fn from(err: redis::RedisError) -> Self {
        ConnectionError::ClientCreation(err)
    }
}

/// 结果类型别名
pub type Result<T> = std::result::Result<T, ConnectionError>;
