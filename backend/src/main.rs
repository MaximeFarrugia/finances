use std::{time::Duration, sync::Arc};

use anyhow::Context;
use derive_getters::Getters;
use sqlx::{PgPool, postgres::{PgPoolOptions, PgConnectOptions}};
use tokio::{task, time::sleep, sync::Mutex};

use crate::models::company::Company;

mod router;
mod stock;
mod models;

#[derive(Getters, Clone)]
pub struct AppState {
    reqwest_client: reqwest::Client,
    pg_pool: PgPool
}

async fn create_pg_pool() -> anyhow::Result<PgPool> {
    let host = std::env::var("POSTGRES_HOST").unwrap_or("localhost".to_owned());
    let port = std::env::var("POSTGRES_PORT").unwrap_or("5432".to_owned());
    let username = std::env::var("POSTGRES_USER").unwrap_or("postgres".to_owned());
    let password = std::env::var("POSTGRES_PASSWORD").context("env var `POSTGRES_PASSWORD` must be set.")?;
    let database = std::env::var("POSTGRES_DB").unwrap_or("postgres".to_owned());
    let max_conn = std::env::var("POSTGRES_MAX_CONN").unwrap_or("50".to_owned());
    let connect_options = PgConnectOptions::new()
        .host(host.as_str())
        .port(port.parse().unwrap_or(5432))
        .username(username.as_str())
        .password(password.as_str())
        .database(database.as_str());
    let pg_pool = PgPoolOptions::new()
        .max_connections(max_conn.parse().unwrap_or(50))
        .connect_with(connect_options)
        .await?;

    return Ok(pg_pool);
}

async fn init_database(state: AppState) -> anyhow::Result<()> {
    let companies = stock::data_fetcher::edgar::cik::get_tickers().await?;

    for company in companies.iter() {
        sqlx::query("insert into companies (cik, ticker, title) values ($1, $2, $3) on conflict (cik) do update set ticker = excluded.ticker, title = excluded.title")
            .bind(company.cik_str())
            .bind(company.ticker())
            .bind(company.title())
            .execute(state.pg_pool())
            .await?;
    }
    return Ok(());
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().context("Should initialize dotenvy successfully.")?;

    let state = AppState {
        reqwest_client: reqwest::Client::new(),
        pg_pool: create_pg_pool().await.context("Creating PgPool.")?
    };
    let address = "127.0.0.1:3001";
    let app = router::create_router(state.clone());

    task::spawn(async move {
        init_database(state).await.unwrap();
    });

    println!("Server listening on {}", address);
    axum::Server::bind(&address.parse().context("Parse server bind address.")?)
        .serve(app.into_make_service())
        .await
        .context("Bind server.")?;

    return Ok(());
}
