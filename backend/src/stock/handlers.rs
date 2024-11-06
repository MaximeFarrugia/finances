use axum::{extract::{Query, State}, Json};
use derive_getters::Getters;
use serde::Deserialize;
use serde_json::{json, Value};
use crate::AppState;
use crate::models::company::Company;

#[derive(Debug, Getters, Deserialize)]
pub struct SearchStockParams {
    q: String,
}

pub async fn search_stock(
    Query(params): Query<SearchStockParams>,
    State(state): State<AppState>
) -> Json<Value> {
    let companies_from_db = sqlx::query_as::<_, Company>("select * from companies where ticker ilike $1 or title ilike $1 order by ticker asc")
        .bind(format!("%{}%", params.q()))
        .fetch_all(state.pg_pool())
        .await;

    return Json(json!(companies_from_db.unwrap()));
}
