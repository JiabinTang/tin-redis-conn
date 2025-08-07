# Tin-redis-conn

## 功能特性

- 🚀 高性能连接管理（基于 Redis connection-manager）
- 🛠️ 完整的 Redis 数据类型支持（字符串、哈希、列表、集合、有序集合）
- 📦 内置 JSON 序列化/反序列化支持
- 🎯 类型安全的泛型 API
- 🔄 批量操作支持
- ⚙️ 灵活的连接配置
- 🌐 异步操作支持（基于 tokio）
- 📝 完整的文档和示例nnection

一个功能丰富、易于使用的 Rust Redis 异步连接库，基于 Redis connection-manager 和 tokio-comp。

## 安装

在 `Cargo.toml` 中添加依赖：

```toml
[dependencies]
tin-redis-conn = "0.1.0"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

## 快速开始

### 基本用法

```rust
use tin_redis_conn::{RedisConnector, RedisUtils, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // 创建连接管理器
    let connection_manager = RedisConnector::new()
        .host("localhost")
        .port(6379)
        .password("")
        .db(0)
        .connection_manager()
        .await?;

    // 创建工具实例
    let redis = RedisUtils::new(connection_manager);

    // 字符串操作
    redis.set("key", "value").await?;
    let value: Option<String> = redis.get("key").await?;
    println!("Value: {:?}", value);

    Ok(())
}
```

## 许可证

本项目采用 MIT 许可证 - 详见 [LICENSE](LICENSE) 文件

## 贡献指南

欢迎贡献代码！请遵循以下步骤：

1. Fork 本仓库
2. 创建功能分支 (`git checkout -b feature/amazing-feature`)
3. 提交更改 (`git commit -m 'Add some amazing feature'`)
4. 推送到分支 (`git push origin feature/amazing-feature`)
5. 创建 Pull Request

## 支持

如果您遇到问题或有建议，请：

- 查看 [examples/utils_example.rs](examples/utils_example.rs) 获取使用示例
- 在 [Issues](https://github.com/your-repo/tin-redis-conn/issues) 中报告问题
- 提交 Pull Request 来改进项目

---

**注意**: 请确保 Redis 服务器在运行示例之前已经启动。
