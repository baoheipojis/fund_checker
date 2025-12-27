# FUND Checker

自动监控市场基金的脚本工具，从 API 获取数据并筛选出溢价高且限购的基金，然后发送通知。

因为只有溢价高且限购的基金，才适合散户进行套利。

## 功能

- 从市场 API 获取基金数据
- 筛选溢价高的基金
- 识别限购基金
- 发送通知提醒


## 构建

```bash
cargo build --release
```

## 运行

```bash
cargo run
```

## 格式化代码

```bash
cargo fmt
```

## 代码检查

```bash
cargo clippy
```

## License

MIT
