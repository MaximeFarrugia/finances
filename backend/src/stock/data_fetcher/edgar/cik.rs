use derive_getters::Getters;
use serde::Deserialize;
use serde_json::{Map, Value, from_value};

#[derive(Debug, Deserialize, Getters)]
pub struct Company {
    cik_str: i64,
    ticker: String,
    title: String,
}

const URI: &'static str = "https://www.sec.gov/files/company_tickers.json";

pub async fn get_tickers() -> anyhow::Result<Vec<Company>> {
    let client = reqwest::Client::builder().build()?;
    let tickers = client.get(URI).send().await?;
    let tickers = tickers.json::<Map<String, Value>>().await?;
    let tickers: Vec<Company> = tickers
        .iter()
        .fold(vec![], |mut acc, x| {
            if let Ok(ticker) = from_value::<Company>(x.1.clone()) {
                acc.push(ticker);
            }
            acc
        });

    return Ok(tickers);
}
