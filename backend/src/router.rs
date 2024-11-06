use axum::Router;
use crate::{stock::router::create_stock_router, AppState};

pub fn create_router(state: AppState) -> Router {
    return Router::new()
        .nest("/stock", create_stock_router(state));
}
