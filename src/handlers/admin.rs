#![allow(warnings, unused)]
use std::ops::Deref;

use axum::{extract::State, http::StatusCode, response::IntoResponse};
use axum_client_ip::TrueClientIp;
use axum_sessions::extractors::{ReadableSession, WritableSession};
use entity::admin_mac;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

use crate::AppState;

use super::status::status;
#[cfg(not(debug_assertions))]
pub async fn admin(
    State(state): State<AppState>,
    ip: TrueClientIp,
    mut session: WritableSession,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    use crate::responders::file;

    let is_admin = session.get::<bool>("admin");
    let is_admin = match is_admin {
        Some(_) => true,
        None => {
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

            let admin = admin_mac::Entity::find()
                .filter(admin_mac::Column::Mac.contains(&mac))
                .one(&state.connection)
                .await
                .unwrap();

            admin.is_some()
        }
    };

    if !is_admin {
        return Err((
            StatusCode::FORBIDDEN,
            "You are not an admin, this incident will be reported".to_owned(),
        ));
    }

    Ok(file::simple_open("./sites/index/admin/index.html".parse().unwrap()).await)
}

#[cfg(debug_assertions)]
pub async fn admin() -> Result<impl IntoResponse, (StatusCode, String)> {
    use crate::responders::file;

    Ok(file::simple_open("./sites/index/admin/index.html".parse().unwrap()).await)
}
