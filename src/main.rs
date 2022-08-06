use axum::{
    extract::Path,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use tokio;

use log::warn;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::SocketAddr;

mod bourso_scrap;

async fn test() {
    let quotes = HashMap::from([
        ("IE00BYVJRP78", "iShares MSCI EM SRI ETF USD Acc"),
        ("FR0007054358", "Lyxor Euro Stoxx 50 DR ETF Acc"),
        ("FR0000974149", "Oddo BHF Avenir Europe CR-EUR"),
        ("LU1681049018", "Amundi IS S&P 500 ETF C USD"),
        ("LU1829221024", "Lyxor Nasdaq 100 ETF Acc"),
        ("LU0594300096", "Fidelity China Consumer A-Acc-EUR"),
        ("LU0171289902", "BGF Sustainable Energy A2 EUR"),
        ("LU1864483166", "Candriam Eqs L Onclgy Impct R EUR Cap"),
        ("LU1279334210", "Pictet - Robotics P EUR"),
        ("LU0270905242", "Pictet-Security R EUR"),
    ]);

    for (k, v) in quotes.iter() {
        match bourso_scrap::scrap::scrape_isin(k).await {
            Ok(quote) => println!("ETF: {}, Quote: {}", v, quote),
            Err(_) => println!("Fail getting quote for: {} - {}", v, k),
        };
    }
}

async fn root() -> &'static str {
    "Hello, World!"
}

async fn get_isin_value(Path(isin): Path<String>) -> Json<ISIN> {
    let res = bourso_scrap::scrap::scrape_isin(&isin).await;

    Json(ISIN {
        value: res.unwrap_or_default().parse::<f64>().unwrap_or_default(),
    })
}

#[derive(Serialize)]
struct ISIN {
    value: f64,
}

#[tokio::main]
async fn main() {
    env_logger::init();
    //test();
    let app = Router::new()
        .route("/", get(root))
        .route("/isin/:isin", get(get_isin_value));
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
