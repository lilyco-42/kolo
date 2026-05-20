# Rust TUI 库调研 & Postman 替代品分析报告

> 日期: 2026-05-05
> 目标: 寻找 Rust TUI 方案以构建类似 Postman 的工具，优先寻找现有方案

---

## 一、Rust TUI 框架现状

### 1️⃣ Ratatui — 行业标准 (推荐)

| 指标 | 数据 |
|------|------|
| 最新版本 | v0.30.0 (2025-12) |
| 月下载量 | ~300 万 |
| 直接依赖数 | 3,900+ crates |
| GitHub Stars | 14,000+ |
| 后端支持 | crossterm(跨平台), termion(Unix), termwiz |

Ratatui 是目前 Rust TUI 的**绝对霸主**，是原 `tui-rs` 的社区活跃分支。采用**Widget 组件化渲染模型**（表格、图表、段落、列表、仪表等），灵活的布局系统，跨平台支持。文档成熟，社区活跃，模板生成器完善 (`cargo generate ratatui/templates`)。

```toml
[dependencies]
ratatui = "0.30"
crossterm = "0.28"
```

### 2️⃣ 其他值得关注的框架

| 框架 | 特点 | 适用场景 |
|------|------|---------|
| **iocraft** | 声明式/React-like 风格 | 喜欢组件化思维 |
| **weavetui** (v0.1.3) | 基于 Ratatui + Tokio + `#[component]` 宏 | 想要开箱即用的组件框架 |
| **appcui** (v0.3.7) | 全功能框架，内置按钮/菜单/树形视图 | 需要高阶 UI 组件 |
| **Cursive** | ncurses 传统风格 | Unix 偏好的开发者 |

### 框架选型结论

> **选 Ratatui。** 生态最强、文档最全、社区最活跃。如果要写就选它。

---

## 二、已存在的 Rust Postman 替代品

### 🥇 ATAC — 终端里的 Postman（强烈推荐）

| 指标 | 数据 |
|------|------|
| GitHub | [Julien-cpsn/ATAC](https://github.com/Julien-cpsn/ATAC) |
| Stars | **3,500+** |
| 最新版本 | **v0.23.0** (2026-02-08) |
| 安装 | `cargo install atac` |
| 许可证 | MIT |
| 技术栈 | **Ratatui** + reqwest + tokio |

**功能完整度：非常高**

###### 已支持
- 全部 HTTP 方法（包括 TRACE、CONNECT — 超越 Postman）
- Headers、Query、Body（multipart/json/xml/纯文本等）
- Auth（Basic、Bearer、Digest、JWT 编解码）
- **JavaScript 脚本**（pre-request / post-request，使用 boa_engine）
- **WebSocket 客户端**（文本/二进制/ping/pong/close）
- 导入：Postman v2.1 集合、OpenAPI 规范、cURL
- 导出：cURL、PHP Guzzle、Node.js Axios、**Rust Reqwest**
- 语法高亮（syntect）、图片渲染（ratatui-image）
- Vim 键绑定 & 可重映射
- 本地 JSON/YAML 文件存储（可 Git 版本管理）
- 无账号、无云、无遥测、完全离线

###### 路线图
- v1.0.0: Markdown 文档、Insomnia 导入、GraphQL
- v2.0.0: MQTT、gRPC、自动补全

**评价：** 功能极其完善，代码质量高，更新活跃。如果你只是想用 Postman 替代品，**直接安装 ATAC**。

---

### 🥈 其他 Rust 方案一览

| 项目 | 类型 | 安装 | 特点 |
|------|------|------|------|
| **Nexus** | TUI | `cargo install nexus-tui` | Vim 风格导航，Postman 导入/导出，sled 嵌入式数据库 |
| **CuTE** | TUI | `cargo install cute_tui` | API 密钥管理（SQLite），Postman 导入，JSON 美化 |
| **lazycurl** | TUI | 源码编译 | 环境变量/密钥脱敏，可配置键绑定 |
| **Netbook** | TUI | 源码编译 | 支持 YAML/JSON 集合，headless CI 模式，插件系统 |
| **endptr** | TUI | 源码编译 | 轻量级 HTTP TUI |

### 🖥️ 桌面端（非 TUI，但也是 Rust）

| 项目 | 技术栈 | Stars | 特点 |
|------|--------|-------|------|
| **Yaak** | Tauri (Rust) + React | 17,900+ | REST/GraphQL/WebSocket/gRPC/SSE，本地优先，Git 友好。由 Insomnia 原作者开发。商业使用需付费许可 |
| **Bruno** | Electron (非 Rust) | 28,000+ | 离线优先，Git 版本管理，纯文本集合格式 |

---

## 三、结论与建议

### 要不要自己写？

| 场景 | 建议 |
|------|------|
| **只是想用 Postman 替代品** | **直接用 ATAC。** 功能够用，成熟稳定。`cargo install atac` 一分钟搞定 |
| **想学 Rust + TUI 练手** | 可以自己写，框架选 **Ratatui**。功能范围缩小到核心 HTTP 请求/响应展示 |
| **有 ATAC 未满足的定制需求** | 可以在 ATAC 提 PR/Issue，或基于 Ratatui 从零写 |

### 如果决定自己写，推荐技术栈

```
ratatui + crossterm  → TUI 框架
reqwest              → HTTP 客户端
tokio                → 异步运行时
serde/serde_json     → 序列化
clap                 → CLI 参数解析
sled / SQLite        → 本地持久化
syntect              → 语法高亮
```

### 自写需要实现的核心功能

1. URL 输入栏 + Method 选择器
2. Headers / Query Params / Body 编辑区
3. 响应展示区（状态码、Headers、Body 语法高亮）
4. 集合管理（Collection / 文件夹 / 请求）
5. 环境变量系统
6. 请求历史

这是一个**2-4 周**的全职工作量，取决于功能范围。

---

**一句话总结：** ATAC 已经非常成熟，建议直接用。如果想学 Ratatui 练手也可以写，但不要期望短期内超越 ATAC 的功能完整度。
