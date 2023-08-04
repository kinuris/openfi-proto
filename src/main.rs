use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;

use axum::routing::post;
use axum::{routing::get, Router};
use axum_sessions::{async_session::MemoryStore, SessionLayer};
use openfi_proto::client_timer::{assure_auth_client_state, decrement_active_user};
use tokio::sync::Mutex;

use migration::{Migrator, MigratorTrait};
use openfi_proto::{handlers, AppState, ClientConnections};
use sea_orm::Database;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let connection = Database::connect(dotenv::var("DATABASE_URL").unwrap())
        .await
        .unwrap();
    Migrator::up(&connection, None).await.unwrap();
    println!("Migrations ran");

    let active_clients: ClientConnections = Arc::new(Mutex::new(HashMap::new()));

    // Separate thread
    tokio::spawn(decrement_active_user(
        connection.clone(),
        active_clients.clone(),
    ));

    // Separate thread
    tokio::spawn(assure_auth_client_state(active_clients.clone()));

    let session_layer = SessionLayer::new(
        MemoryStore::new(),
        dotenv::var("SECRET").unwrap().as_bytes(),
    )
    .with_secure(false);

    let app = Router::new()
        .route("/", get(handlers::index::index))
        .route("/status/", get(handlers::status::status))
        .route(
            "/request-access/",
            post(handlers::request_access::request_access),
        )
        .route("/pause/", post(handlers::pause::pause))
        .nest_service("/assets/", ServeDir::new("./sites/assets"))
        .with_state(AppState {
            active_clients,
            connection,
        })
        .layer(session_layer);

    axum::Server::bind(&"0.0.0.0:7766".parse().unwrap())
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap()
}
