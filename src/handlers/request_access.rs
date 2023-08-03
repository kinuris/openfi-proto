use std::process::Command;

use axum::{extract::State, http::StatusCode};
use axum_client_ip::TrueClientIp;
use axum_sessions::extractors::WritableSession;
use entity::client;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

use crate::AppState;

use super::status::status;

pub async fn request_access(
    ip: TrueClientIp,
    State(state): State<AppState>,
    // State(client_connections): State<ClientConnections>,
    mut session: WritableSession,
) -> Result<(), (StatusCode, &'static str)> {
    let mac = session.get::<String>("mac");
    let mac = match mac {
        Some(mac) => mac,
        None => {
            let data = status(ip).await.unwrap();
            session.insert("mac", &data.mac).unwrap();

            data.mac.clone()
        }
    };

    let client = client::Entity::find()
        .filter(client::Column::Mac.contains(&mac))
        .one(&state.connection)
        .await
        .unwrap()
        .ok_or_else(|| (StatusCode::FORBIDDEN, "Forbidden: Report sent to admin"))?; // TODO:

    if client.remaining_seconds <= 0 {
        return Err((
            StatusCode::FORBIDDEN,
            "No Remaining Seconds: Purchase a plan",
        ));
    }

    let mut active_map = state.active_clients.lock().await;

    if active_map.contains_key(&client.mac) {
        return Err((StatusCode::FORBIDDEN, "Already Active"));
    }

    active_map.insert(client.mac.clone(), client.remaining_seconds);

    tokio::task::spawn_blocking(move || {
        Command::new("ndsctl").arg("auth").arg(mac).spawn().unwrap()
    })
    .await
    .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Server Error"))?;

    Ok(())
}
