use crate::api::common::{parse_apply_status_limit, parse_percent};
use crate::models::Fund;
use serde::Deserialize;
use std::convert::TryFrom;

/// QDII API 响应
#[derive(Debug, Deserialize)]
pub struct QdiiResponse {
    pub rows: Vec<QdiiRow>,
    #[allow(dead_code)]
    pub page: usize,
}

/// QDII 基金行数据
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct QdiiRow {
    pub id: String,
    pub cell: QdiiCell,
}

/// QDII 基金单元格数据
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct QdiiCell {
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

/// 从 QDII 行数据转换为统一的基金模型
impl TryFrom<&QdiiRow> for Fund {
    type Error = String;

    fn try_from(row: &QdiiRow) -> Result<Self, Self::Error> {
        let premium_rate = parse_percent(&row.cell.discount_rt)
            .ok_or_else(|| format!("Invalid discount_rt: {}", row.cell.discount_rt))?;

        let purchase_limit = parse_apply_status_limit(&row.cell.apply_status);

        Ok(Fund::new_qdii(
            row.cell.fund_id.clone(),
            row.cell.fund_nm.clone(),
            premium_rate,
            purchase_limit,
        ))
    }
}
