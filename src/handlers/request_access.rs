#![allow(warnings, unused)]
use std::process::Command;

use axum::{extract::State, http::StatusCode};
use axum_client_ip::TrueClientIp;
use axum_sessions::extractors::WritableSession;
use entity::client;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

use crate::AppState;

use super::status::status;

#[cfg(not(debug_assertions))]
pub async fn request_access(
    ip: TrueClientIp,
    State(state): State<AppState>,
    mut session: WritableSession,
) -> Result<(), (StatusCode, String)> {
    let mac = session.get::<String>("mac");
    let mac = match mac {
        Some(mac) => mac,
        None => {
            let data = status(ip).await.map_err(|(s, h)| {
                // TODO: Send email to admin

                (s, h.0)
            })?;
            session.insert("mac", &data.mac).unwrap();

            data.mac.clone()
        }
    };

    let client = client::Entity::find()
        .filter(client::Column::Mac.contains(&mac))
        .one(&state.connection)
        .await
        .unwrap()
        .ok_or_else(|| {
            (
                StatusCode::FORBIDDEN,
                "Forbidden: Report sent to admin".to_owned(),
            )
        })?; // TODO:

    if client.remaining_seconds <= 0 {
        return Err((
            StatusCode::FORBIDDEN,
            "No Remaining Seconds: Purchase a plan".to_owned(),
        ));
    }

    let mut active_map = state.active_clients.lock().await;

    if active_map.contains_key(&client.mac) {
        return Err((StatusCode::FORBIDDEN, "Already Active".to_owned()));
    }

    active_map.insert(client.mac.clone(), client.remaining_seconds);

    // TODO: Create ndsctl rust wrapper (with NDSCtlConfig struct)

    tokio::task::spawn_blocking(move || {
        Command::new("ndsctl")
            .arg("auth")
            .arg(mac)
            .spawn()
            .unwrap()
            .wait()
            .unwrap();
    })
    .await
    .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Server Error".to_owned()))?;

    Ok(())
}

#[cfg(debug_assertions)]
pub async fn request_access() -> Result<(), (StatusCode, String)> {
    Ok(())
}
