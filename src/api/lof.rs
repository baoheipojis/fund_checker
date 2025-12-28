use crate::api::common::{parse_apply_status_limit, parse_percent};
use crate::models::Fund;
use serde::Deserialize;
use std::convert::TryFrom;

/// LOF API 响应（股票型和指数型共享相同结构）
#[derive(Debug, Deserialize)]
pub struct LofResponse {
    pub rows: Vec<LofRow>,
    #[allow(dead_code)]
    pub page: usize,
}

/// LOF 基金行数据
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct LofRow {
    pub id: String,
    pub cell: LofCell,
}

/// LOF 基金单元格数据
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct LofCell {
    #[serde(rename = "fund_id")]
    pub fund_id: String,

    #[serde(rename = "fund_nm")]
    pub fund_nm: String,

    #[serde(rename = "price")]
    pub price: Option<String>,

    #[serde(rename = "discount_rt")]
    pub discount_rt: String,

    #[serde(rename = "min_amt")]
    pub min_amt: Option<String>,

    #[serde(rename = "apply_status")]
    pub apply_status: String,

    #[serde(rename = "redeem_status")]
    pub redeem_status: Option<String>,

    #[serde(default)]
    #[serde(rename = "apply_redeem_status")]
    pub apply_redeem_status: Option<String>,
}

/// 从 LOF 行数据转换为统一的基金模型
impl TryFrom<&LofRow> for Fund {
    type Error = String;

    fn try_from(row: &LofRow) -> Result<Self, Self::Error> {
        let premium_rate = parse_percent(&row.cell.discount_rt)
            .ok_or_else(|| format!("Invalid discount_rt: {}", row.cell.discount_rt))?;

        let purchase_limit = parse_apply_status_limit(&row.cell.apply_status);

        Ok(Fund::new_lof(
            row.cell.fund_id.clone(),
            row.cell.fund_nm.clone(),
            premium_rate,
            purchase_limit,
        ))
    }
}
