#![allow(warnings, unused)]
use std::process::Command;

use axum::{http::StatusCode, response::Html, Json};
use axum_client_ip::TrueClientIp;

use crate::RawClientData;

#[cfg(not(debug_assertions))]
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

#[cfg(debug_assertions)]
pub async fn status() -> Result<Json<RawClientData>, (StatusCode, Html<String>)> {
    let data = "{
        \"gatewayname\":\"openNDS%20Node%3a7cc2c6480d3b%20\",
        \"gatewayaddress\":\"192.168.1.1:2050\",
        \"gatewayfqdn\":\"status.client\",
        \"version\":\"10.1.1\",
        \"client_type\":\"cpd_can\",
        \"mac\":\"20:16:d8:15:b1:26\",
        \"ip\":\"192.168.1.131\",
        \"clientif\":\"enx7cc2c6480d3bend0\",
        \"session_start\":\"1691112539\",
        \"session_end\":\"1691198939\",
        \"last_active\":\"1691113598\",
        \"token\":\"a3f4112d\",
        \"state\":\"Authenticated\",
        \"custom\":\"bmE=\",
        \"download_rate_limit_threshold\":\"null\",
        \"download_packet_rate\":\"null\",
        \"download_bucket_size\":\"null\",
        \"upload_rate_limit_threshold\":\"null\",
        \"upload_packet_rate\":\"null\",
        \"upload_bucket_size\":\"null\",
        \"download_quota\":\"null\",
        \"upload_quota\":\"null\",
        \"download_this_session\":\"244223\",
        \"download_session_avg\":\"1889.21\",
        \"upload_this_session\":\"7023\",
        \"upload_session_avg\":\"54.33\"
      }";
    let data = serde_json::from_str::<RawClientData>(data).unwrap();

    Ok(Json(data))
}
