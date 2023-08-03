use std::process::Command;

use axum::{http::StatusCode, response::Html, Json};
use axum_client_ip::TrueClientIp;

use crate::RawClientData;

pub async fn status(
    TrueClientIp(ip): TrueClientIp,
) -> Result<Json<RawClientData>, (StatusCode, Html<String>)> {
    let output = tokio::task::spawn_blocking(move || {
        Command::new("ndsctl")
        .arg("json")
        .arg(ip.to_string())
        .output()
        .unwrap()
    })
    .await
    .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "<a href=\"http://192.168.1.1:8080/\"http://192.168.1.1:8080/ (Error: Command Failed)</a>".to_owned().into()))?;

    let data = serde_json::from_str::<RawClientData>(std::str::from_utf8(&output.stdout).unwrap())
        .unwrap();

    Ok(Json(data))
}
