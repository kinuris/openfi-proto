#![allow(warnings, unused)]
use std::process::Command;

use axum::{
    body::Body,
    extract::State,
    http::{Request, StatusCode},
    response::IntoResponse,
};
use axum_client_ip::TrueClientIp;
use axum_sessions::extractors::WritableSession;
use entity::client;
use sea_orm::{ActiveModelTrait, ActiveValue, ColumnTrait, EntityTrait, QueryFilter};

use crate::{responders::file, AppState, RawClientData};

#[cfg(not(debug_assertions))]
pub async fn index(
    mut session: WritableSession,
    State(state): State<AppState>,
    TrueClientIp(ip): TrueClientIp,
    req: Request<Body>,
) -> Result<impl IntoResponse, StatusCode> {
    let mac = session.get::<String>("mac");
    let mac = match mac {
        Some(mac) => mac,
        None => {
            let output = tokio::task::spawn_blocking(move || {
                Command::new("ndsctl")
                    .arg("json")
                    .arg(ip.to_string())
                    .output()
                    .unwrap()
            })
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

            let parsed_output = std::str::from_utf8(&output.stdout).unwrap();
            let parsed_output = serde_json::from_str::<RawClientData>(parsed_output)
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

            session.insert("mac", &parsed_output.mac).unwrap();

            parsed_output.mac
        }
    };

    let possible_client = client::Entity::find()
        .filter(client::Column::Mac.contains(&mac))
        .one(&state.connection)
        .await
        .unwrap();

    if possible_client.is_none() {
        let client = client::ActiveModel {
            mac: ActiveValue::Set(mac.clone()),
            credits: ActiveValue::Set(0),
            remaining_seconds: ActiveValue::Set(0),
            ..Default::default()
        };

        client.insert(&state.connection).await.unwrap();
    }

    Ok(file::simple_open("./sites/index.html".parse().unwrap()).await)
}

#[cfg(debug_assertions)]
pub async fn index(req: Request<Body>) -> Result<impl IntoResponse, StatusCode> {
    Ok(file::open("./sites/index.html".parse().unwrap(), &req).await)
}
