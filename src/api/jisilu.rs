use crate::api::commodity::CommodityResponse;
use crate::api::lof::LofResponse;
use crate::api::qdii::QdiiResponse;
use crate::models::Fund;
use anyhow::{Context, Result};
use reqwest::Client;
use std::convert::TryFrom;
use std::time::Duration;

/// 集思录 API 客户端
pub struct JisiluClient {
    client: Client,
    base_url: String,
}

/// 基金类型枚举，用于 API 调用
#[derive(Clone, Copy)]
enum FundApiType {
    Qdii,
    Commodity,
    StockLof,
    IndexLof,
}

impl FundApiType {
    fn url_path(&self) -> &'static str {
        match self {
            FundApiType::Qdii => "/data/qdii/qdii_list/E",
            FundApiType::Commodity => "/data/qdii/qdii_list/C",
            FundApiType::StockLof => "/data/lof/stock_lof_list/",
            FundApiType::IndexLof => "/data/lof/index_lof_list/",
        }
    }

    fn referer(&self) -> &'static str {
        match self {
            FundApiType::Qdii | FundApiType::Commodity => "https://www.jisilu.cn/data/qdii/",
            FundApiType::StockLof | FundApiType::IndexLof => "https://www.jisilu.cn/data/lof/",
        }
    }
}

impl JisiluClient {
    pub fn new() -> Result<Self> {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .user_agent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7)")
            .cookie_store(true)
            .build()
            .context("Failed to create HTTP client")?;

        Ok(Self {
            client,
            base_url: "https://www.jisilu.cn".to_string(),
        })
    }

    /// 通用的 API 列表获取方法，返回 JSON 文本
    async fn fetch_list(&self, api_type: FundApiType, page: u32) -> Result<String> {
        let url = format!("{}{}", self.base_url, api_type.url_path());
        let timestamp = chrono::Utc::now().timestamp_millis();

        let response = self
            .client
            .get(&url)
            .query(&[
                ("___jsl", "LST"),
                ("___t", &timestamp.to_string()),
                ("rp", "55"),
                ("page", &page.to_string()),
            ])
            .header("Accept", "application/json, text/javascript, */*; q=0.01")
            .header("Referer", api_type.referer())
            .send()
            .await
            .context("Failed to send request to Jisilu API")?;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!(
                "API returned error status: {}",
                response.status()
            ));
        }

        let text = response
            .text()
            .await
            .context("Failed to read response body")?;

        // 处理可能的 JSONP 包装
        let json_text = if text.starts_with("/*-secure-") {
            text.lines()
                .skip(1)
                .collect::<Vec<_>>()
                .join("")
                .trim_end_matches(';')
                .to_string()
        } else {
            text
        };

        Ok(json_text)
    }

    /// 通用的分页获取方法 - QDII
    async fn fetch_all_qdii_internal(&self) -> Result<Vec<Fund>> {
        let mut all_funds = Vec::new();
        let mut page = 1;
        let mut last_row_count = usize::MAX;

        loop {
            let json_text = self.fetch_list(FundApiType::Qdii, page).await?;
            let response: QdiiResponse =
                serde_json::from_str(&json_text).context("Failed to parse JSON response")?;

            if response.rows.is_empty() {
                break;
            }

            for row in &response.rows {
                if let Ok(fund) = Fund::try_from(row) {
                    all_funds.push(fund);
                }
            }

            if response.rows.len() < last_row_count {
                break;
            }

            last_row_count = response.rows.len();
            page += 1;

            if page > 200 {
                break;
            }

            tokio::time::sleep(Duration::from_millis(500)).await;
        }

        Ok(all_funds)
    }

    /// 通用的分页获取方法 - Commodity
    async fn fetch_all_commodity_internal(&self) -> Result<Vec<Fund>> {
        let mut all_funds = Vec::new();
        let mut page = 1;
        let mut last_row_count = usize::MAX;

        loop {
            let json_text = self.fetch_list(FundApiType::Commodity, page).await?;
            let response: CommodityResponse =
                serde_json::from_str(&json_text).context("Failed to parse JSON response")?;

            if response.rows.is_empty() {
                break;
            }

            for row in &response.rows {
                if let Ok(fund) = Fund::try_from(row) {
                    all_funds.push(fund);
                }
            }

            if response.rows.len() < last_row_count {
                break;
            }

            last_row_count = response.rows.len();
            page += 1;

            if page > 200 {
                break;
            }

            tokio::time::sleep(Duration::from_millis(500)).await;
        }

        Ok(all_funds)
    }

    /// 通用的分页获取方法 - LOF (支持多个 API 类型)
    async fn fetch_all_lof_internal(&self, api_types: &[FundApiType]) -> Result<Vec<Fund>> {
        let mut all_funds = Vec::new();

        for &api_type in api_types {
            let mut page = 1;
            let mut last_row_count = usize::MAX;

            loop {
                let json_text = self.fetch_list(api_type, page).await?;
                let response: LofResponse =
                    serde_json::from_str(&json_text).context("Failed to parse JSON response")?;

                if response.rows.is_empty() {
                    break;
                }

                for row in &response.rows {
                    if let Ok(fund) = Fund::try_from(row) {
                        all_funds.push(fund);
                    }
                }

                if response.rows.len() < last_row_count {
                    break;
                }

                last_row_count = response.rows.len();
                page += 1;

                if page > 200 {
                    break;
                }

                tokio::time::sleep(Duration::from_millis(500)).await;
            }
        }

        Ok(all_funds)
    }

    /// 获取所有 QDII 基金数据（自动分页）
    pub async fn fetch_all_qdii(&self) -> Result<Vec<Fund>> {
        self.fetch_all_qdii_internal().await
    }

    /// 获取所有商品基金数据（自动分页）
    pub async fn fetch_all_commodity(&self) -> Result<Vec<Fund>> {
        self.fetch_all_commodity_internal().await
    }

    /// 获取所有 LOF 基金数据（自动分页，合并股票型和指数型）
    pub async fn fetch_all_lof(&self) -> Result<Vec<Fund>> {
        self.fetch_all_lof_internal(&[FundApiType::StockLof, FundApiType::IndexLof])
            .await
    }
}
