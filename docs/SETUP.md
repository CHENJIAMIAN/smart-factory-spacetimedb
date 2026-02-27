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

## 3. Edge 运行模式（已打通）

`edge-gateway` 现在支持两种模式：

- 本地实时文件模式（默认）：每秒写 `dashboard/public/live.json`，不依赖 SpacetimeDB 服务。
- Reducer 调用模式：设置 `SPACETIME_CALL_ENABLED=true` 后，额外执行 `spacetime call ... sensor_update`。

## 4. Dashboard 实时接线（已打通）

`dashboard/src/main.tsx` 已替换 mock 数据：

- 每 1 秒轮询 `/live.json`
- 渲染设备状态卡片 + 告警列表

## 5. Demo 验收标准

- 每秒有采集数据上报
- 温度 > 85 触发 critical 告警
- 前端 1 秒内可见状态与告警变化
