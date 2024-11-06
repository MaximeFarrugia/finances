use async_trait::async_trait;
use crate::{stock::Stock, AppState};
use super::StockDataFetcher;

pub struct Nasdaq;

const USER_AGENT: &'static str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/114.0.0.0 Safari/537.36";

#[async_trait]
impl StockDataFetcher for Nasdaq {
    async fn search(state: AppState, ticker: &str) -> anyhow::Result<Vec<Stock>> {
        let api_url = "https://api.nasdaq.com/api/autocomplete/slookup/10";
        let res = state.reqwest_client()
            .get(api_url)
            .query(&[("search", ticker)])
            .header("user-agent", USER_AGENT)
            .send()
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
