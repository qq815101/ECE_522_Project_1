use yahoo_finance_api as yahoo;

// A single trading day's prices, shared with analysis and charting modules.
#[derive(Debug, Clone)]
pub struct StockDay {
    pub timestamp: i64,
    pub date: String, // YYYY-MM-DD (UTC)
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
}

// Normalize a ticker and reject clearly malformed input before sending a request.
pub fn normalize_ticker(input: &str) -> Result<String, String> {
    let ticker = input.trim().to_ascii_uppercase();

    if ticker.is_empty()
        || !ticker
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '.' | '-' | '^' | '=' | '_'))
    {
        return Err(format!(
            "Invalid ticker '{input}'. Try a symbol such as AAPL, MSFT, or SHOP.TO."
        ));
    }

    Ok(ticker)
}

// Download the last six months of daily OHLC prices from Yahoo Finance.
// Network/API errors, invalid symbols, and empty results are returned as Err,
// never unwrapped or panicked on.
pub async fn fetch_six_months(ticker: &str) -> Result<Vec<StockDay>, String> {
    let provider = yahoo::YahooConnector::new()
        .map_err(|e| format!("Failed to initialize Yahoo Finance client: {e}"))?;

    let response = provider
        .get_quote_range(ticker, "1d", "6mo")
        .await
        .map_err(|e| format!("Could not download prices for '{ticker}': {e}"))?;

    let quotes = response
        .quotes()
        .map_err(|e| format!("Could not read prices for '{ticker}': {e}"))?;

    let mut days: Vec<StockDay> = quotes
        .into_iter()
        .filter(|q| {
            q.open.is_finite()
                && q.high.is_finite()
                && q.low.is_finite()
                && q.close.is_finite()
                && q.high >= q.low
                && q.low > 0.0
                && q.close > 0.0
        })
        .filter_map(|q| {
            let date = yahoo::time::OffsetDateTime::from_unix_timestamp(q.timestamp)
                .ok()?
                .date()
                .to_string();

            Some(StockDay {
                timestamp: q.timestamp,
                date,
                open: q.open,
                high: q.high,
                low: q.low,
                close: q.close,
            })
        })
        .collect();

    days.sort_by_key(|day| day.timestamp);
    days.dedup_by_key(|day| day.timestamp);

    if days.is_empty() {
        return Err(format!(
            "No usable daily prices found for '{ticker}'. Check the ticker or try again later."
        ));
    }

    Ok(days)
}

#[cfg(test)]
mod tests {
    use super::normalize_ticker;

    #[test]
    fn uppercase_and_trim() {
        assert_eq!(normalize_ticker(" aapl ").unwrap(), "AAPL");
        assert_eq!(normalize_ticker("shop.to").unwrap(), "SHOP.TO");
    }

    #[test]
    fn accept_common_ticker_punctuation() {
        assert_eq!(normalize_ticker("BRK-B").unwrap(), "BRK-B");
        assert_eq!(normalize_ticker("^GSPC").unwrap(), "^GSPC");
    }

    #[test]
    fn reject_empty_or_invalid_tickers() {
        assert!(normalize_ticker("  ").is_err());
        assert!(normalize_ticker("AAPL ???").is_err());
    }
}
