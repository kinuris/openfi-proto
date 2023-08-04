use std::process::Command;

use axum::{extract::State, http::StatusCode};
use axum_client_ip::TrueClientIp;
use axum_sessions::extractors::WritableSession;

use crate::AppState;

use super::status::status;

pub async fn pause(
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

    let mut active_clients = state.active_clients.lock().await;
    if !active_clients.contains_key(&mac) {
        return Err((StatusCode::FORBIDDEN, "Connection Not Active".to_owned()));
    }

    active_clients.remove_entry(&mac);

    tokio::task::spawn_blocking(move || {
        Command::new("ndsctl")
            .arg("deauth")
            .arg(mac)
            .spawn()
            .unwrap()
    })
    .await
    .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Server Error".to_owned()))?;

    Ok(())
}
