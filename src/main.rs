use axum::{
    http::Method,
    routing::{get, post},
    Router,
};
use dotenv::dotenv;
use service::solana_service::{get_balance, get_sols, transact_sol};
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber;

mod model;
mod service;
mod util;

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let cors = CorsLayer::new()
        // allow `GET` and `POST` when accessing the resource
        .allow_methods([Method::GET, Method::POST])
        .allow_headers(Any)
        // allow requests from any origin
        .allow_origin(Any);

    // build our application with a single route
    let app = Router::new()
        .route(
            "/getBalance",
            get(|| async { get_balance().await.to_string() }),
        )
        .route("/getSols", get(get_sols))
        .route("/transferSols", post(transact_sol))
        .layer(cors);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("🚀 Server listening on http://127.0.0.1:3000");

    axum::serve(listener, app).await.unwrap();
}
