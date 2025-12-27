# Rust 练习


## 功能列表

### cli 命令行工具

- csv 转为 json, yaml, toml
- 生成随机密码
- base64 编码/解码
- 使用 Blake3 哈希，使用 Ed25519 签名/验证
- 文件服务器

### 多线程
- 多线程的使用
- 简单的 redis server 实现

### 生态系统
- 错误处理
- 日志处理：jaeger 日志
- Serde 序列化/反序列化
- tokio 异步编程
- BytesMut 的使用
- 简单 nginx 服务器
- sqlx 的简单使用
- 聊天服务器
- crm 微服务（TLS/nginx）
- 处理数据
    - 使用 arrow 处理 ndjson 数据

### chat 聊天服务


## 安装工具

### 安装 Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 安装 VSCode 插件

- crates: Rust 包管理
- Even Better TOML: TOML 文件支持
- Better Comments: 优化注释显示
- Error Lens: 错误提示优化
- GitLens: Git 增强
- Github Copilot: 代码提示
- indent-rainbow: 缩进显示优化
- Prettier - Code formatter: 代码格式化
- REST client: REST API 调试
- rust-analyzer: Rust 语言支持
- Rust Test lens: Rust 测试支持
- Rust Test Explorer: Rust 测试概览
- TODO Highlight: TODO 高亮
- vscode-icons: 图标优化
- YAML: YAML 文件支持

### 安装 cargo generate

cargo generate 是一个用于生成项目模板的工具。它可以使用已有的 github repo 作为模版生成新的项目。

```bash
cargo install cargo-generate
```

在我们的课程中，新的项目会使用 `tyr-rust-bootcamp/template` 模版生成基本的代码：

```bash
cargo generate tyr-rust-bootcamp/template
```

### 安装 pre-commit

在使用 pip 之前，可使用 uv 管理 python

pre-commit 是一个代码检查工具，可以在提交代码前进行代码检查。

```bash
uv pip install pre-commit
```

安装成功后运行 `uv run pre-commit install` 即可。

### 安装 Cargo deny

Cargo deny 是一个 Cargo 插件，可以用于检查依赖的安全性。

```bash
cargo install --locked cargo-deny
cargo deny init
cargo deny fetch
cargo deny check
```

### 安装 typos

typos 是一个拼写检查工具。

```bash
cargo install typos-cli
```

### 安装 git cliff

git cliff 是一个生成 changelog 的工具。

```bash
cargo install git-cliff
git cliff --init
```

### 安装 cargo nextest

cargo nextest 是一个 Rust 增强测试工具。

```bash
cargo install cargo-nextest --locked
```

### 安装 sqlx-cli

sqlx-cli 是一个用于 SQL 数据库的命令行工具。

```bash
cargo install sqlx-cli --no-default-features --features 'rustls,postgres'
```

### 简化版安装
运行 `setup_tools.ps1` 脚本即可安装所有工具。

## 其他工具安装

### Protoc
Manually install protoc from [here](https://github.com/protocolbuffers/protobuf/releases).
