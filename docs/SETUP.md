# SETUP / 对齐步骤

## 1. 对齐 SpacetimeDB 版本

由于 SpacetimeDB CLI 与 Rust SDK 版本更新快，先执行：

```bash
spacetime --version
```

然后按官方文档替换以下依赖：

- `spacetimedb-module/Cargo.toml` 的 `spacetimedb`
- `edge-gateway/Cargo.toml` 的 `spacetimedb-sdk`

## 2. 发布模块

```bash
cd spacetimedb-module
spacetime publish smart-factory
```

## 3. 接通 edge reducer 调用

在 `edge-gateway/src/main.rs` 中替换 pseudo 代码为真实 SDK 调用：

- 连接 endpoint
- 调用 `sensor_update(machine_id, sensor_type, value, timestamp)`

## 4. 接通 dashboard 订阅

将 `dashboard/src/main.tsx` 中 mock 数据改为 SDK 实时订阅：

- 订阅 `machine`
- 订阅 `alert`
- 渲染状态卡片 + 告警滚动区

## 5. Demo 验收标准

- 每秒有采集数据上报
- 温度 > 85 触发 critical 告警
- 前端 1 秒内可见状态与告警变化
