use async_trait::async_trait;
use serde::Deserialize;
use crate::{stock::Stock, AppState};
use super::StockDataFetcher;

pub struct YahooFinance;

#[derive(Deserialize, Debug)]
struct ChartResponse {
    // chart: Chart
}

#[derive(Deserialize, Debug)]
pub struct Chart {
    result: ChartResult
    // error: ChartError
}

#[derive(Deserialize, Debug)]
pub struct ChartResult {
    meta: ChartMeta,
    timestamp: Vec<u64>,
    // events: ChartEvents,
    // indicators: ChartIndicators,
}

#[derive(Deserialize, Debug)]
pub struct ChartError {
    code: String,
    description: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ChartMeta {
    currency: String,
    symbol: String,
    exchange_name: String,
    instrument_type: String,
    first_trade_date: i64,
    regular_market_time: i64,
    gmtoffset: i64,
    timezone: String,
    exchange_timezone_name: String,
    regular_market_price: f64,
    chart_previous_close: f64,
    #[serde(default)]
    previous_close: Option<f64>,
    #[serde(default)]
    scale: Option<i32>,
    price_hint: i64,
    current_trading_period: TradingPeriod,
    #[serde(default)]
    trading_periods: Option<Vec<Vec<PeriodInfo>>>,
    data_granularity: String,
    range: String,
    valid_ranges: Vec<String>
}

#[derive(Deserialize, Debug)]
pub struct TradingPeriod {
    pub pre: PeriodInfo,
    pub regular: PeriodInfo,
    pub post: PeriodInfo,
}

#[derive(Deserialize, Debug)]
pub struct PeriodInfo {
    pub timezone: String,
    pub start: u32,
    pub end: u32,
    pub gmtoffset: i32,
}

#[async_trait]
impl StockDataFetcher for YahooFinance {
    async fn search(state: AppState, ticker: &str) -> anyhow::Result<Vec<Stock>> {
        let api_url = "https://query1.finance.yahoo.com/v8/finance/chart/".to_owned() + ticker;
        let res = state.reqwest_client()
            .get(api_url)
            .query(&[
                ("formatted", "true"),
                ("includeAdjustedClose", "false"),
                ("interval", "1mo"),
                ("period1", "0"),
                ("period2", chrono::offset::Utc::now().timestamp().to_string().as_str()),
                ("events", "div"),
                ("useYfid", "true"),
            ])
            .send()
            .await?
            .json()
            .await?;
        println!("res: {:?}", res);
        let stock = Stock {
            name: "wef".to_owned(),
            ticker: "wef".to_owned(),
            exchange: None,
            sector: None,
            industry: None,
        };
        return Ok(vec![stock]);
    }
}
