use crate::config::FilterRule;
use crate::models::Fund;

/// 根据配置规则筛选基金
pub fn filter_funds<'a>(funds: &'a [Fund], config: &FilterRule) -> Vec<&'a Fund> {
    funds
        .iter()
        .filter(|fund| {
            // 溢价率高于阈值 且 申购限额低于配置值
            fund.premium_rate > config.premium_threshold
                && fund.purchase_limit <= config.purchase_limit
        })
        .collect()
}

/// 显示筛选结果
pub fn display_results(funds: &[&Fund]) {
    if funds.is_empty() {
        println!("没有找到符合条件的基金。");
        return;
    }

    println!("找到 {} 只符合条件的基金：\n", funds.len());

    for fund in funds {
        let fund_type_str = match fund.fund_type {
            crate::models::FundType::Qdii => "QDII",
            crate::models::FundType::Lof => "LOF",
            crate::models::FundType::Commodity => "商品",
        };

        println!(
            "[{}] {} | {} | 溢价率: {}% | 限购: {} 元",
            fund_type_str, fund.fund_id, fund.fund_name, fund.premium_rate, fund.purchase_limit
        );
    }
}
