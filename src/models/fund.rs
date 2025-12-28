use rust_decimal::Decimal;

/// 基金类型枚举
#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum FundType {
    Qdii,
    Lof,
    Commodity,
}

/// 统一的基金领域模型
#[derive(Debug, Clone)]
pub struct Fund {
    pub fund_type: FundType,
    pub fund_id: String,
    pub fund_name: String,
    pub premium_rate: Decimal,
    pub purchase_limit: u64,
}

impl Fund {
    /// 创建新的 QDII 基金
    #[allow(dead_code)]
    pub fn new_qdii(
        fund_id: String,
        fund_name: String,
        premium_rate: Decimal,
        purchase_limit: u64,
    ) -> Self {
        Self {
            fund_type: FundType::Qdii,
            fund_id,
            fund_name,
            premium_rate,
            purchase_limit,
        }
    }

    /// 创建新的 LOF 基金
    #[allow(dead_code)]
    pub fn new_lof(
        fund_id: String,
        fund_name: String,
        premium_rate: Decimal,
        purchase_limit: u64,
    ) -> Self {
        Self {
            fund_type: FundType::Lof,
            fund_id,
            fund_name,
            premium_rate,
            purchase_limit,
        }
    }

    /// 创建新的商品基金
    pub fn new_commodity(
        fund_id: String,
        fund_name: String,
        premium_rate: Decimal,
        purchase_limit: u64,
    ) -> Self {
        Self {
            fund_type: FundType::Commodity,
            fund_id,
            fund_name,
            premium_rate,
            purchase_limit,
        }
    }
}
