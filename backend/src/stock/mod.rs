use serde::{Serialize, Deserialize};

pub mod router;
pub mod data_fetcher;
pub mod handlers;

#[derive(Debug, Serialize, Deserialize)]
pub struct Stock {
    name: String,
    ticker: String,
    exchange: Option<String>,
    sector: Option<String>,
    industry: Option<String>,
}
