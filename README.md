# Smart Factory Digital Twin (SpacetimeDB MVP)

目标：2 周内跑通“传感器 → 实时状态镜像 → 告警 → 大屏可视化”的最小可演示链路。

## 目录

- `spacetimedb-module/`：SpacetimeDB Rust 模块（表 + reducer）
- `edge-gateway/`：边缘采集器（模拟/桥接传感器数据）
- `dashboard/`：前端大屏（先占位，后接真实订阅）
- `docs/`：设计与实施文档

## MVP 范围（第一阶段）

1. 设备状态表（Machine）
2. 传感器时序表（SensorData）
3. 告警表（Alert）
4. `sensor_update` reducer：写入、聚合、阈值告警
5. Edge 模拟器：每秒上报温度/振动
6. Dashboard：展示设备状态卡片 + 告警列表

## 本地启动计划

> 注意：SpacetimeDB API 迭代较快。以当前官方 CLI/SDK 为准，若编译报错，按 `docs/SETUP.md` 的“对齐步骤”处理。

### 1) 安装工具

```bash
cargo install spacetimedb-cli
spacetime login
```

### 2) 初始化并发布模块

```bash
cd spacetimedb-module
# 视本机 CLI 版本确认 init 命令参数
spacetime publish smart-factory
```

### 3) 启动边缘采集器

```bash
cd ../edge-gateway
cargo run
```

### 4) 启动大屏

```bash
cd ../dashboard
pnpm install
pnpm dev
```

## 下一步（第二阶段）

- OEE 拆分为 Availability / Performance / Quality 三因子
- 新增定时 reducer 做预测性维护
- 引入 OPC UA / MQTT adapter
- 接入 3D 产线（Three.js）
