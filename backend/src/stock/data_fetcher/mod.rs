mod nasdaq;
mod yahoo_finance;
pub mod edgar;

use async_trait::async_trait;
use crate::AppState;

use super::Stock;

pub use yahoo_finance::YahooFinance;

#[async_trait]
pub trait StockDataFetcher {
    async fn search(state: AppState, ticker: &str) -> anyhow::Result<Vec<Stock>>;
}
