# FUND Checker

> 自动监控市场基金溢价的 Rust 工具，帮助散户发现套利机会

## 背景

在基金市场中，部分基金（如 QDII、商品基金、LOF）存在溢价和限购现象。**溢价高且限购少**的基金适合散户进行溢价套利，即申购后场内卖出，具体流程自行研究。

本项目通过自动获取集思录（Jisilu）的实时基金数据，筛选出符合条件的套利机会。

## 功能特性

- 支持多种基金类型：QDII 基金、商品基金、LOF 基金
- 实时获取市场数据：从集思录 API 获取最新的基金溢价率、申购限额等信息
- 灵活筛选规则：可配置溢价率阈值和申购限额

## 筛选逻辑

只有同时满足以下条件的基金才会被标记为套利机会：

1. 溢价率 ≥ 配置的阈值（默认 2%）
2. 单日申购限额 ≤ 配置的上限（默认 1000 元）

## 快速开始

### 环境要求

- Rust 1.85+ (edition 2024)
- 网络连接（访问集思录 API）
- 支持平台：Windows / macOS / Linux

### 安装

```bash
# 克隆仓库
git clone https://github.com/baoheipojis/fund_checker.git
cd fund_checker

# 构建项目
cargo build --release
```

### 配置

在项目根目录创建 `config.toml` 文件：

```toml
[filter_rule]
# 溢价率阈值（%），只有溢价高于此值才触发
premium_threshold = 2.0
# 申购限额（元），只有限购低于此值才触发
purchase_limit = 1000
```

### 运行

```bash
# 直接运行
cargo run

# 或运行编译后的二进制文件
./target/release/fund_checker
```

### 输出示例

```
溢价率阈值: 2%
申购限额: 1000 元

正在获取各类基金数据...

共获取 48 只 QDII 基金
共获取 13 只商品基金
共获取 260 只 LOF 基金

共获取 321 只基金

找到 5 只符合条件的基金：

[QDII] 161128 | 标普信息科技LOF | 溢价率: 5.27% | 限购: 10 元
[QDII] 161130 | 纳斯达克100LOF | 溢价率: 2.51% | 限购: 10 元
[商品] 161129 | 原油LOF易方达 | 溢价率: 3.09% | 限购: 10 元
[商品] 160416 | 石油基金LOF | 溢价率: 2.95% | 限购: 500 元
[LOF] 161226 | 国投白银LOF | 溢价率: 19.41% | 限购: 100 元
```

## 项目结构

```
fund_checker/
├── src/
│   ├── main.rs           # 程序入口
│   ├── api/              # API 客户端模块
│   │   ├── mod.rs        # 模块导出
│   │   ├── jisilu.rs     # 集思录 API 客户端
│   │   ├── common.rs     # 通用类型定义
│   │   ├── qdii.rs       # QDII 基金 API
│   │   ├── commodity.rs  # 商品基金 API
│   │   └── lof.rs        # LOF 基金 API
│   ├── config.rs         # 配置加载
│   ├── filter.rs         # 基金筛选逻辑
│   └── models/           # 数据模型
│       ├── mod.rs
│       └── fund.rs       # 基金数据结构
├── config.toml           # 配置文件
├── Cargo.toml            # 项目依赖
└── README.md
```

## 开发

### 格式化代码

```bash
cargo fmt
```

### 代码检查

```bash
cargo clippy
```

### 运行测试

```bash
cargo test
```

## 技术栈

- **异步运行时**: Tokio
- **HTTP 客户端**: Reqwest
- **序列化**: Serde + Serde_json
- **配置解析**: TOML
- **精确计算**: rust_decimal

## 注意事项

1. 本工具仅提供数据筛选，不构成任何投资建议
2. 套利存在风险，请根据自身情况谨慎决策
3. 集思录 API 可能随时变更，如有问题请检查 API 接口

## License

MIT
