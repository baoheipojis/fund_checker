//! 通用解析函数模块

/// 解析百分比字符串 (如 "5.25%" -> Decimal 5.25)
///
/// 支持负数："-5.25%" -> Decimal -5.25
pub fn parse_percent(s: &str) -> Option<rust_decimal::Decimal> {
    let binding = s.trim().trim_end_matches('%').replace(',', "");
    let cleaned = binding.trim();
    cleaned.parse::<rust_decimal::Decimal>().ok()
}

/// 解析申购状态字符串获取限额
/// "开放申购" -> u64::MAX (无限额)
/// "暂停申购" -> u64::MAX (无限额)
/// "限10" -> 10
/// "限100" -> 100
/// "限100万" -> 1000000
/// "限10万" -> 100000
/// "限1千" -> 1000
pub fn parse_apply_status_limit(s: &str) -> u64 {
    if s == "开放申购" || s == "暂停申购" {
        u64::MAX - 1 // 无限额，用大数表示
    } else if let Some(rest) = s.strip_prefix("限") {
        // 提取 "限" 后面的数字和单位
        if let Some(num_str) = rest.strip_suffix("万") {
            // "10万" -> 100000
            num_str
                .parse::<f64>()
                .map_or(u64::MAX - 1, |n| (n * 10000.0) as u64)
        } else if let Some(num_str) = rest.strip_suffix("千") {
            // "1千" -> 1000
            num_str
                .parse::<f64>()
                .map_or(u64::MAX - 1, |n| (n * 1000.0) as u64)
        } else {
            // "100" -> 100
            rest.parse::<u64>().unwrap_or(u64::MAX - 1)
        }
    } else {
        // 其他情况也视为无限额
        u64::MAX - 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_percent() {
        assert_eq!(
            parse_percent("5.25%"),
            Some(rust_decimal::Decimal::try_from(5.25).unwrap())
        );
        assert_eq!(
            parse_percent("-5.25%"),
            Some(rust_decimal::Decimal::try_from(-5.25).unwrap())
        );
        assert_eq!(parse_percent("10%"), Some(rust_decimal::Decimal::from(10)));
        assert_eq!(
            parse_percent("1,234.56%"),
            Some(rust_decimal::Decimal::try_from(1234.56).unwrap())
        );
        assert_eq!(parse_percent("invalid"), None);
    }

    #[test]
    fn test_parse_apply_status_limit() {
        assert_eq!(parse_apply_status_limit("开放申购"), u64::MAX - 1);
        assert_eq!(parse_apply_status_limit("暂停申购"), u64::MAX - 1);
        assert_eq!(parse_apply_status_limit("限10"), 10);
        assert_eq!(parse_apply_status_limit("限100"), 100);
        assert_eq!(parse_apply_status_limit("限1000"), 1000);
        assert_eq!(parse_apply_status_limit("限1千"), 1000);
        assert_eq!(parse_apply_status_limit("限10万"), 100000);
        assert_eq!(parse_apply_status_limit("限100万"), 1000000);
    }
}
