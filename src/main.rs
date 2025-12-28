mod api;
mod config;
mod filter;
mod models;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // 加载配置
    let config = config::load_config();
    println!("溢价率阈值: {}%", config.filter_rule.premium_threshold);
    println!("申购限额: {} 元\n", config.filter_rule.purchase_limit);

    // 初始化 API 客户端
    let client = api::JisiluClient::new()?;

    // 获取所有 QDII 基金数据
    println!("正在获取 QDII 基金数据...");
    let qdii_funds = client.fetch_all_qdii().await?;
    println!("共获取 {} 只 QDII 基金", qdii_funds.len());

    // 获取所有商品基金数据
    println!("正在获取商品基金数据...");
    let commodity_funds = client.fetch_all_commodity().await?;
    println!("共获取 {} 只商品基金", commodity_funds.len());

    // 获取所有 LOF 基金数据
    println!("正在获取 LOF 基金数据...");
    let lof_funds = client.fetch_all_lof().await?;
    println!("共获取 {} 只 LOF 基金\n", lof_funds.len());

    // 合并所有基金数据
    let mut all_funds = Vec::new();
    all_funds.extend(qdii_funds);
    all_funds.extend(commodity_funds);
    all_funds.extend(lof_funds);
    println!("共获取 {} 只基金\n", all_funds.len());

    // 筛选符合条件的基金
    let filtered = filter::filter_funds(&all_funds, &config.filter_rule);

    // 显示结果
    filter::display_results(&filtered);

    Ok(())
}
