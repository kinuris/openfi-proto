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

pub async fn index(
    mut session: WritableSession,
    State(state): State<AppState>,
    TrueClientIp(ip): TrueClientIp,
    req: Request<Body>,
) -> Result<impl IntoResponse, StatusCode> {
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

    let parsed_output = serde_json::from_str::<RawClientData>(parsed_output);
    let mac = match parsed_output {
        Ok(data) => data.mac,
        Err(_) => {
            let mac = session.get::<String>("mac");

            if mac.is_some() {
                mac.unwrap()
            } else {
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            }
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

    let mac_exists = session.get::<String>("mac").is_some();
    if !mac_exists {
        session.insert("mac", &mac).unwrap();
    }

    Ok(file::open("./sites/index.html".parse().unwrap(), &req)
        .await
        .unwrap())
}
