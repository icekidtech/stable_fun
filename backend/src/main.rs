use axum::{routing::post, Router};
mod routes;
mod services;
mod config;
mod utils;

#[tokio::main]
async fn main() {
    let app = Router::new()
    .route("/mint", post(routes::mint_stablecoins))
    .route("/redeem", post(routes::redeem_stablecoins));

    let config = config::load_config();

    println!("Server running on {}", config.server_address);
    axum::Server::bind(&config.server_address.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}