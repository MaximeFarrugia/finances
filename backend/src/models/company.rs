use serde::{Deserialize, Serialize};

use crate::stock::data_fetcher::edgar::cik;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Company {
    cik: i64,
    ticker: String,
    title: String,
}

impl From<cik::Company> for Company {
    fn from(value: cik::Company) -> Self {
        return Self {
            cik: value.cik_str().to_owned(),
            ticker: value.ticker().to_owned(),
            title: value.title().to_owned(),
        };
    }
}
