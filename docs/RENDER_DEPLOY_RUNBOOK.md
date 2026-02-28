# Smart Factory（SpacetimeDB）Render 部署 Runbook

## 结论先说
Render 可以部署，但推荐走 **Docker + Persistent Disk + 独立前端静态托管**。

---

## 1. 前置条件

1) GitHub 仓库可访问（建议 public）
2) Render 账号 + 可创建 Web Service
3) 项目包含：
- `spacetimedb-module/`
- `edge-gateway/`
- `dashboard/`

---

## 2. 最小架构

- Service A（Web Service）: SpacetimeDB + module（后端）
- Service B（Web Service / Worker）: edge-gateway（上报）
- Service C（Static Site）: dashboard（前端）

> MVP 简化：先上 A + C，B 可先本地跑或同机进程跑。

---

## 3. 环境变量（建议）

### A: spacetimedb-service
- `SPACETIME_URI` = 服务内URL（如 `http://0.0.0.0:3000`）
- `SPACETIME_MODULE` = `smart-factory`

### B: edge-gateway
- `SPACETIME_URI` = A 的内网地址
- `SPACETIME_MODULE` = `smart-factory`
- `MACHINE_ID` = `1`
- `SPACETIME_CALL_ENABLED` = `true`

### C: dashboard
- `VITE_API_BASE` = A 的公网地址（如 `https://xxx.onrender.com`）

---

## 4. 部署步骤（MVP顺序）

1) 先部署 A（SpacetimeDB）
   - 启动成功后开放健康检查路径（如 `/health`）
2) 发布 module 到 A
   - 执行：`spacetime publish smart-factory --server <render-server>`
3) 部署 B（edge-gateway）
   - 确认上报日志有 `sensor_update` 成功记录
4) 部署 C（dashboard）
   - 页面能读取实时状态与告警

---

## 5. 验收清单

- [ ] A 服务可访问
- [ ] module 发布成功
- [ ] B 每秒上报温度/振动
- [ ] 超阈值（温度>85 / 振动>60）可触发告警
- [ ] C 页面 1s 内看到状态变化

---

## 6. 回滚策略

- 代码回滚：Render 选择上一版 Deploy
- 配置回滚：恢复上一版环境变量
- 数据回滚：Persistent Disk 快照（如有）

---

## 7. 常见故障

1) **前端有界面但不变动**
- 检查 B 是否在上报
- 检查 C 的 API 地址是否正确

2) **有上报但无告警**
- 检查 reducer 阈值逻辑
- 检查 module 版本是否已发布

3) **部署成功但WS断连**
- 检查 Render 端口与代理
- 检查是否允许长连接
