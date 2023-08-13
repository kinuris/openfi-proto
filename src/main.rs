use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;

use axum::http::Method;
use axum::routing::post;
use axum::{routing::get, Router};
use axum_sessions::{async_session::MemoryStore, SessionLayer};
use openfi_proto::client_timer::{assure_auth_client_state, decrement_active_user};
use tokio::sync::Mutex;

use migration::{Migrator, MigratorTrait};
use openfi_proto::{handlers, AppState, ClientConnections};
use sea_orm::Database;
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let connection = Database::connect(dotenv::var("DATABASE_URL").unwrap())
        .await
        .unwrap();
    Migrator::up(&connection, None).await.unwrap();
    println!("Migrations ran");

    let active_clients: ClientConnections = Arc::new(Mutex::new(HashMap::new()));

    if cfg!(not(debug_assertions)) {
        // Separate thread
        tokio::spawn(decrement_active_user(
            connection.clone(),
            active_clients.clone(),
        ));

        // Separate thread
        tokio::spawn(assure_auth_client_state(active_clients.clone()));
    }

    let session_layer = SessionLayer::new(
        MemoryStore::new(),
        dotenv::var("SECRET").unwrap().as_bytes(),
    )
    .with_secure(false);

    let mut app = Router::new()
        .route("/", get(handlers::index::index))
        .route("/admin", get(handlers::admin::admin))
        .route("/status", get(handlers::status::status))
        .route("/data", get(handlers::data::data))
        .route("/data-stream", get(handlers::data::data_stream))
        .route("/get-plans", get(handlers::plans::plans))
        .route(
            "/request-access",
            post(handlers::request_access::request_access),
        )
        .route("/spend/:kind/:id", post(handlers::credits::spend_credits))
        .route("/pause", post(handlers::pause::pause))
        .route("/gen-code/:kind/:units", post(handlers::code::gen_code))
        .route("/get-codes", post(handlers::code::get_codes))
        .route("/redeem-code", post(handlers::code::redeem_code))
        .nest_service("/index", ServeDir::new("./sites/index"))
        .with_state(AppState {
            active_clients,
            connection,
        })
        .layer(session_layer);

    if cfg!(debug_assertions) {
        let cors = CorsLayer::new()
            .allow_methods([Method::GET, Method::POST])
            .allow_headers(Any)
            .allow_origin(Any);

        app = app.layer(cors);
    }

    axum::Server::bind(&"0.0.0.0:7766".parse().unwrap())
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap()
}
