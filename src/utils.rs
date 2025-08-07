use crate::error::{ConnectionError, Result};
use redis::aio::ConnectionManager;
use redis::{AsyncCommands, FromRedisValue, ToRedisArgs};
use serde::{Deserialize, Serialize};

/// Redis 工具类 - 提供常用的异步 Redis 操作方法
pub struct RedisUtils;

impl RedisUtils {
    // ==================== 字符串操作 ====================

    /// 设置字符串值
    ///
    /// # Arguments
    ///
    /// * `key` - 键名
    /// * `value` - 值
    ///
    /// # Returns
    ///
    /// 返回操作结果
    pub async fn set<K, V>(conn: &mut ConnectionManager, key: K, value: V) -> Result<()>
    where
        K: ToRedisArgs + Send + Sync,
        V: ToRedisArgs + Send + Sync,
    {
        let _: () = conn.set(key, value).await?;
        Ok(())
    }

    /// 设置字符串值并指定过期时间
    ///
    /// # Arguments
    ///
    /// * `key` - 键名
    /// * `value` - 值
    /// * `seconds` - 过期时间（秒）
    ///
    /// # Returns
    ///
    /// 返回操作结果
    pub async fn setex<K, V>(
        conn: &mut ConnectionManager,
        key: K,
        value: V,
        seconds: usize,
    ) -> Result<()>
    where
        K: ToRedisArgs + Send + Sync,
        V: ToRedisArgs + Send + Sync,
    {
        let _: () = conn.set_ex(key, value, seconds as u64).await?;
        Ok(())
    }

    /// 获取字符串值
    ///
    /// # Arguments
    ///
    /// * `key` - 键名
    ///
    /// # Returns
    ///
    /// 返回值或 None
    pub async fn get<K, V>(conn: &mut ConnectionManager, key: K) -> Result<Option<V>>
    where
        K: ToRedisArgs + Send + Sync,
        V: FromRedisValue,
    {
        let result: Option<V> = conn.get(key).await?;
        Ok(result)
    }

    /// 删除键
    ///
    /// # Arguments
    ///
    /// * `keys` - 要删除的键名列表
    ///
    /// # Returns
    ///
    /// 返回删除的键数量
    pub async fn del<K>(conn: &mut ConnectionManager, keys: K) -> Result<i32>
    where
        K: ToRedisArgs + Send + Sync,
    {
        let result: i32 = conn.del(keys).await?;
        Ok(result)
    }

    /// 检查键是否存在
    ///
    /// # Arguments
    ///
    /// * `key` - 键名
    ///
    /// # Returns
    ///
    /// 返回是否存在
    pub async fn exists<K>(conn: &mut ConnectionManager, key: K) -> Result<bool>
    where
        K: ToRedisArgs + Send + Sync,
    {
        let result: bool = conn.exists(key).await?;
        Ok(result)
    }

    /// 设置键的过期时间
    ///
    /// # Arguments
    ///
    /// * `key` - 键名
    /// * `seconds` - 过期时间（秒）
    ///
    /// # Returns
    ///
    /// 返回操作结果
    pub async fn expire<K>(conn: &mut ConnectionManager, key: K, seconds: usize) -> Result<bool>
    where
        K: ToRedisArgs + Send + Sync,
    {
        let result: bool = conn.expire(key, seconds as i64).await?;
        Ok(result)
    }

    /// 获取键的剩余生存时间
    ///
    /// # Arguments
    ///
    /// * `key` - 键名
    ///
    /// # Returns
    ///
    /// 返回剩余秒数，-1表示永不过期，-2表示键不存在
    pub async fn ttl<K>(conn: &mut ConnectionManager, key: K) -> Result<i32>
    where
        K: ToRedisArgs + Send + Sync,
    {
        let result: i32 = conn.ttl(key).await?;
        Ok(result)
    }

    // ==================== 哈希表操作 ====================

    /// 设置哈希表字段值
    ///
    /// # Arguments
    ///
    /// * `key` - 哈希表键名
    /// * `field` - 字段名
    /// * `value` - 值
    ///
    /// # Returns
    ///
    /// 返回操作结果
    pub async fn hset<K, F, V>(
        conn: &mut ConnectionManager,
        key: K,
        field: F,
        value: V,
    ) -> Result<bool>
    where
        K: ToRedisArgs + Send + Sync,
        F: ToRedisArgs + Send + Sync,
        V: ToRedisArgs + Send + Sync,
    {
        let result: bool = conn.hset(key, field, value).await?;
        Ok(result)
    }

    /// 获取哈希表字段值
    ///
    /// # Arguments
    ///
    /// * `key` - 哈希表键名
    /// * `field` - 字段名
    ///
    /// # Returns
    ///
    /// 返回字段值或 None
    pub async fn hget<K, F, V>(conn: &mut ConnectionManager, key: K, field: F) -> Result<Option<V>>
    where
        K: ToRedisArgs + Send + Sync,
        F: ToRedisArgs + Send + Sync,
        V: FromRedisValue,
    {
        let result: Option<V> = conn.hget(key, field).await?;
        Ok(result)
    }

    /// 获取哈希表所有字段和值
    ///
    /// # Arguments
    ///
    /// * `key` - 哈希表键名
    ///
    /// # Returns
    ///
    /// 返回字段值映射
    pub async fn hgetall<K>(
        conn: &mut ConnectionManager,
        key: K,
    ) -> Result<std::collections::HashMap<String, String>>
    where
        K: ToRedisArgs + Send + Sync,
    {
        let result: std::collections::HashMap<String, String> = conn.hgetall(key).await?;
        Ok(result)
    }

    /// 删除哈希表字段
    ///
    /// # Arguments
    ///
    /// * `key` - 哈希表键名
    /// * `fields` - 要删除的字段名列表
    ///
    /// # Returns
    ///
    /// 返回删除的字段数量
    pub async fn hdel<K, F>(conn: &mut ConnectionManager, key: K, fields: F) -> Result<i32>
    where
        K: ToRedisArgs + Send + Sync,
        F: ToRedisArgs + Send + Sync,
    {
        let result: i32 = conn.hdel(key, fields).await?;
        Ok(result)
    }

    /// 检查哈希表字段是否存在
    ///
    /// # Arguments
    ///
    /// * `key` - 哈希表键名
    /// * `field` - 字段名
    ///
    /// # Returns
    ///
    /// 返回是否存在
    pub async fn hexists<K, F>(conn: &mut ConnectionManager, key: K, field: F) -> Result<bool>
    where
        K: ToRedisArgs + Send + Sync,
        F: ToRedisArgs + Send + Sync,
    {
        let result: bool = conn.hexists(key, field).await?;
        Ok(result)
    }

    // ==================== 列表操作 ====================

    /// 向列表左侧推入元素
    ///
    /// # Arguments
    ///
    /// * `key` - 列表键名
    /// * `values` - 要推入的值
    ///
    /// # Returns
    ///
    /// 返回列表长度
    pub async fn lpush<K, V>(conn: &mut ConnectionManager, key: K, values: V) -> Result<i32>
    where
        K: ToRedisArgs + Send + Sync,
        V: ToRedisArgs + Send + Sync,
    {
        let result: i32 = conn.lpush(key, values).await?;
        Ok(result)
    }

    /// 向列表右侧推入元素
    ///
    /// # Arguments
    ///
    /// * `key` - 列表键名
    /// * `values` - 要推入的值
    ///
    /// # Returns
    ///
    /// 返回列表长度
    pub async fn rpush<K, V>(conn: &mut ConnectionManager, key: K, values: V) -> Result<i32>
    where
        K: ToRedisArgs + Send + Sync,
        V: ToRedisArgs + Send + Sync,
    {
        let result: i32 = conn.rpush(key, values).await?;
        Ok(result)
    }

    /// 从列表左侧弹出元素
    ///
    /// # Arguments
    ///
    /// * `key` - 列表键名
    ///
    /// # Returns
    ///
    /// 返回弹出的元素或 None
    pub async fn lpop<K, V>(conn: &mut ConnectionManager, key: K) -> Result<Option<V>>
    where
        K: ToRedisArgs + Send + Sync,
        V: FromRedisValue,
    {
        let result: Option<V> = conn.lpop(key, None).await?;
        Ok(result)
    }

    /// 从列表右侧弹出元素
    ///
    /// # Arguments
    ///
    /// * `key` - 列表键名
    ///
    /// # Returns
    ///
    /// 返回弹出的元素或 None
    pub async fn rpop<K, V>(conn: &mut ConnectionManager, key: K) -> Result<Option<V>>
    where
        K: ToRedisArgs + Send + Sync,
        V: FromRedisValue,
    {
        let result: Option<V> = conn.rpop(key, None).await?;
        Ok(result)
    }

    /// 获取列表长度
    ///
    /// # Arguments
    ///
    /// * `key` - 列表键名
    ///
    /// # Returns
    ///
    /// 返回列表长度
    pub async fn llen<K>(conn: &mut ConnectionManager, key: K) -> Result<i32>
    where
        K: ToRedisArgs + Send + Sync,
    {
        let result: i32 = conn.llen(key).await?;
        Ok(result)
    }

    /// 获取列表指定范围的元素
    ///
    /// # Arguments
    ///
    /// * `key` - 列表键名
    /// * `start` - 开始索引
    /// * `stop` - 结束索引
    ///
    /// # Returns
    ///
    /// 返回元素列表
    pub async fn lrange<K>(
        conn: &mut ConnectionManager,
        key: K,
        start: isize,
        stop: isize,
    ) -> Result<Vec<String>>
    where
        K: ToRedisArgs + Send + Sync,
    {
        let result: Vec<String> = conn.lrange(key, start, stop).await?;
        Ok(result)
    }

    // ==================== 集合操作 ====================

    /// 向集合添加成员
    ///
    /// # Arguments
    ///
    /// * `key` - 集合键名
    /// * `members` - 要添加的成员
    ///
    /// # Returns
    ///
    /// 返回添加的成员数量
    pub async fn sadd<K, M>(conn: &mut ConnectionManager, key: K, members: M) -> Result<i32>
    where
        K: ToRedisArgs + Send + Sync,
        M: ToRedisArgs + Send + Sync,
    {
        let result: i32 = conn.sadd(key, members).await?;
        Ok(result)
    }

    /// 从集合移除成员
    ///
    /// # Arguments
    ///
    /// * `key` - 集合键名
    /// * `members` - 要移除的成员
    ///
    /// # Returns
    ///
    /// 返回移除的成员数量
    pub async fn srem<K, M>(conn: &mut ConnectionManager, key: K, members: M) -> Result<i32>
    where
        K: ToRedisArgs + Send + Sync,
        M: ToRedisArgs + Send + Sync,
    {
        let result: i32 = conn.srem(key, members).await?;
        Ok(result)
    }

    /// 检查成员是否在集合中
    ///
    /// # Arguments
    ///
    /// * `key` - 集合键名
    /// * `member` - 成员
    ///
    /// # Returns
    ///
    /// 返回是否存在
    pub async fn sismember<K, M>(conn: &mut ConnectionManager, key: K, member: M) -> Result<bool>
    where
        K: ToRedisArgs + Send + Sync,
        M: ToRedisArgs + Send + Sync,
    {
        let result: bool = conn.sismember(key, member).await?;
        Ok(result)
    }

    /// 获取集合所有成员
    ///
    /// # Arguments
    ///
    /// * `key` - 集合键名
    ///
    /// # Returns
    ///
    /// 返回成员列表
    pub async fn smembers<K>(conn: &mut ConnectionManager, key: K) -> Result<Vec<String>>
    where
        K: ToRedisArgs + Send + Sync,
    {
        let result: Vec<String> = conn.smembers(key).await?;
        Ok(result)
    }

    /// 获取集合成员数量
    ///
    /// # Arguments
    ///
    /// * `key` - 集合键名
    ///
    /// # Returns
    ///
    /// 返回成员数量
    pub async fn scard<K>(conn: &mut ConnectionManager, key: K) -> Result<i32>
    where
        K: ToRedisArgs + Send + Sync,
    {
        let result: i32 = conn.scard(key).await?;
        Ok(result)
    }

    // ==================== 有序集合操作 ====================

    /// 向有序集合添加成员
    ///
    /// # Arguments
    ///
    /// * `key` - 有序集合键名
    /// * `score` - 分数
    /// * `member` - 成员
    ///
    /// # Returns
    ///
    /// 返回添加的成员数量
    pub async fn zadd<K, S, M>(
        conn: &mut ConnectionManager,
        key: K,
        score: S,
        member: M,
    ) -> Result<i32>
    where
        K: ToRedisArgs + Send + Sync,
        S: ToRedisArgs + Send + Sync,
        M: ToRedisArgs + Send + Sync,
    {
        let result: i32 = conn.zadd(key, member, score).await?;
        Ok(result)
    }

    /// 从有序集合移除成员
    ///
    /// # Arguments
    ///
    /// * `key` - 有序集合键名
    /// * `members` - 要移除的成员
    ///
    /// # Returns
    ///
    /// 返回移除的成员数量
    pub async fn zrem<K, M>(conn: &mut ConnectionManager, key: K, members: M) -> Result<i32>
    where
        K: ToRedisArgs + Send + Sync,
        M: ToRedisArgs + Send + Sync,
    {
        let result: i32 = conn.zrem(key, members).await?;
        Ok(result)
    }

    /// 获取有序集合指定范围的成员
    ///
    /// # Arguments
    ///
    /// * `key` - 有序集合键名
    /// * `start` - 开始索引
    /// * `stop` - 结束索引
    ///
    /// # Returns
    ///
    /// 返回成员列表
    pub async fn zrange<K>(
        conn: &mut ConnectionManager,
        key: K,
        start: isize,
        stop: isize,
    ) -> Result<Vec<String>>
    where
        K: ToRedisArgs + Send + Sync,
    {
        let result: Vec<String> = conn.zrange(key, start, stop).await?;
        Ok(result)
    }

    /// 获取有序集合成员数量
    ///
    /// # Arguments
    ///
    /// * `key` - 有序集合键名
    ///
    /// # Returns
    ///
    /// 返回成员数量
    pub async fn zcard<K>(conn: &mut ConnectionManager, key: K) -> Result<i32>
    where
        K: ToRedisArgs + Send + Sync,
    {
        let result: i32 = conn.zcard(key).await?;
        Ok(result)
    }

    // ==================== JSON 操作 (需要序列化/反序列化支持) ====================

    /// 设置 JSON 对象
    ///
    /// # Arguments
    ///
    /// * `key` - 键名
    /// * `value` - 要序列化的对象
    ///
    /// # Returns
    ///
    /// 返回操作结果
    pub async fn set_json<K, V>(conn: &mut ConnectionManager, key: K, value: &V) -> Result<()>
    where
        K: ToRedisArgs + Send + Sync,
        V: Serialize,
    {
        let json_str = serde_json::to_string(value)
            .map_err(|e| ConnectionError::Serialization(e.to_string()))?;
        Self::set(conn, key, json_str).await
    }

    /// 获取 JSON 对象
    ///
    /// # Arguments
    ///
    /// * `key` - 键名
    ///
    /// # Returns
    ///
    /// 返回反序列化的对象或 None
    pub async fn get_json<K, V>(conn: &mut ConnectionManager, key: K) -> Result<Option<V>>
    where
        K: ToRedisArgs + Send + Sync,
        V: for<'de> Deserialize<'de>,
    {
        let json_str: Option<String> = Self::get(conn, key).await?;
        match json_str {
            Some(s) => {
                let value = serde_json::from_str(&s)
                    .map_err(|e| ConnectionError::Deserialization(e.to_string()))?;
                Ok(Some(value))
            }
            None => Ok(None),
        }
    }

    // ==================== 通用结构体操作 ====================

    /// 设置任意结构体对象
    ///
    /// # Arguments
    ///
    /// * `key` - 键名
    /// * `value` - 要存储的结构体对象
    ///
    /// # Returns
    ///
    /// 返回操作结果
    ///
    /// # Examples
    ///
    /// ```
    /// use serde::{Serialize, Deserialize};
    ///
    /// #[derive(Serialize, Deserialize)]
    /// struct User {
    ///     id: u64,
    ///     name: String,
    ///     email: String,
    /// }
    ///
    /// let user = User {
    ///     id: 1,
    ///     name: "张三".to_string(),
    ///     email: "zhangsan@example.com".to_string(),
    /// };
    ///
    /// utils.set_struct("user:1", &user).await?;
    /// ```
    pub async fn set_struct<K, T>(conn: &mut ConnectionManager, key: K, value: &T) -> Result<()>
    where
        K: ToRedisArgs + Send + Sync,
        T: Serialize,
    {
        let json_str = serde_json::to_string(value)
            .map_err(|e| ConnectionError::Serialization(e.to_string()))?;
        Self::set(conn, key, json_str).await
    }

    /// 设置任意结构体对象并指定过期时间
    ///
    /// # Arguments
    ///
    /// * `key` - 键名
    /// * `value` - 要存储的结构体对象
    /// * `seconds` - 过期时间（秒）
    ///
    /// # Returns
    ///
    /// 返回操作结果
    ///
    /// # Examples
    ///
    /// ```
    /// let user = User { /* ... */ };
    /// utils.set_struct_ex("user:1", &user, 3600).await?; // 1小时后过期
    /// ```
    pub async fn set_struct_ex<K, T>(
        conn: &mut ConnectionManager,
        key: K,
        value: &T,
        seconds: usize,
    ) -> Result<()>
    where
        K: ToRedisArgs + Send + Sync,
        T: Serialize,
    {
        let json_str = serde_json::to_string(value)
            .map_err(|e| ConnectionError::Serialization(e.to_string()))?;
        Self::setex(conn, key, json_str, seconds).await
    }

    /// 获取任意结构体对象
    ///
    /// # Arguments
    ///
    /// * `key` - 键名
    ///
    /// # Returns
    ///
    /// 返回反序列化的结构体对象或 None
    ///
    /// # Examples
    ///
    /// ```
    /// let user: Option<User> = utils.get_struct("user:1").await?;
    /// match user {
    ///     Some(u) => println!("用户名: {}", u.name),
    ///     None => println!("用户不存在"),
    /// }
    /// ```
    pub async fn get_struct<K, T>(conn: &mut ConnectionManager, key: K) -> Result<Option<T>>
    where
        K: ToRedisArgs + Send + Sync,
        T: for<'de> Deserialize<'de>,
    {
        let json_str: Option<String> = Self::get(conn, key).await?;
        match json_str {
            Some(s) => {
                let value = serde_json::from_str(&s)
                    .map_err(|e| ConnectionError::Deserialization(e.to_string()))?;
                Ok(Some(value))
            }
            None => Ok(None),
        }
    }

    /// 批量获取值
    ///
    /// # Arguments
    ///
    /// * `keys` - 键名列表
    ///
    /// # Returns
    ///
    /// 返回值列表
    pub async fn mget<K>(conn: &mut ConnectionManager, keys: &[K]) -> Result<Vec<Option<String>>>
    where
        K: ToRedisArgs + Clone + Send + Sync,
    {
        let result: Vec<Option<String>> = conn.get(keys).await?;
        Ok(result)
    }

    /// 批量获取结构体对象
    ///
    /// # Arguments
    ///
    /// * `keys` - 键名列表
    ///
    /// # Returns
    ///
    /// 返回结构体对象列表
    ///
    /// # Examples
    ///
    /// ```
    /// let keys = vec!["user:1", "user:2", "user:3"];
    /// let users: Vec<Option<User>> = utils.mget_struct(&keys).await?;
    /// ```
    pub async fn mget_struct<K, T>(
        conn: &mut ConnectionManager,
        keys: &[K],
    ) -> Result<Vec<Option<T>>>
    where
        K: ToRedisArgs + Clone + Send + Sync,
        T: for<'de> Deserialize<'de>,
    {
        let json_strings: Vec<Option<String>> = Self::mget(conn, keys).await?;
        let mut results = Vec::new();

        for json_str in json_strings {
            match json_str {
                Some(s) => {
                    let value = serde_json::from_str(&s)
                        .map_err(|e| ConnectionError::Deserialization(e.to_string()))?;
                    results.push(Some(value));
                }
                None => results.push(None),
            }
        }

        Ok(results)
    }
}
