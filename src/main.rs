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

    // 并行获取所有类型基金数据
    println!("正在获取各类基金数据...\n");
    let (qdii_funds, commodity_funds, lof_funds) = tokio::try_join!(
        client.fetch_all_qdii(),
        client.fetch_all_commodity(),
        client.fetch_all_lof()
    )?;

    println!("共获取 {} 只 QDII 基金", qdii_funds.len());
    println!("共获取 {} 只商品基金", commodity_funds.len());
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
