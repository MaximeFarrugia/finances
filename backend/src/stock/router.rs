use axum::{Router, routing::get};
use crate::AppState;

use super::handlers::search_stock;

pub fn create_stock_router<S>(state: AppState) -> Router<S> {
    return Router::new()
        .route("/search", get(search_stock))
        .with_state(state);
}
