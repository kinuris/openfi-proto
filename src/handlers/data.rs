#![allow(warnings, unused)]
use std::convert::Infallible;
use std::time::Duration;

use axum::http::HeaderMap;
use axum::response::sse::{Event, KeepAlive};
use axum::{extract::State, http::StatusCode, response::sse::Sse, Json};
use axum_sessions::extractors::{ReadableSession, WritableSession};
use entity::client;
use futures_util::{stream, Stream};
use sea_orm::{ColumnTrait, EntityTrait, IntoActiveModel, QueryFilter};
use tokio_stream::StreamExt;

use crate::AppState;
use crate::ClientData;

#[cfg(not(debug_assertions))]
pub async fn data(
    session: ReadableSession,
    State(state): State<AppState>,
) -> Result<Json<ClientData>, StatusCode> {
    let mac = session
        .get::<String>("mac")
        .ok_or_else(|| StatusCode::NOT_FOUND)?;

    let client = client::Entity::find()
        .filter(client::Column::Mac.contains(&mac))
        .one(&state.connection)
        .await
        .unwrap();

    let model = match client {
        Some(model) => model,
        None => return Err(StatusCode::NOT_FOUND),
    };

    let active_clients = state.active_clients.lock().await;

    let json_data = ClientData {
        id: model.id,
        mac: model.mac,
        credits: model.credits,
        remaining_seconds: model.remaining_seconds,
        active: active_clients.contains_key(&mac),
    };

    Ok(Json(json_data))
}

// NOTE: Error occurs when server is (re)started, while a client still has the webUI open
#[cfg(not(debug_assertions))]
pub async fn data_stream(
    session: ReadableSession,
    State(state): State<AppState>,
) -> (
    HeaderMap,
    Sse<impl futures::Stream<Item = Result<Event, Infallible>>>,
) {
    let mac = session.get::<String>("mac").unwrap(); // TODO: Handle panic

    let stream = futures::stream::unfold((mac, state), |(mac, state)| async {
        let client = client::Entity::find()
            .filter(client::Column::Mac.contains(&mac))
            .one(&state.connection)
            .await
            .unwrap()
            .unwrap();

        let state_copy = state.clone();
        let active_clients = state_copy.active_clients.lock().await;
        let active = active_clients.contains_key(&client.mac);

        Some((
            ClientData {
                remaining_seconds: client.remaining_seconds,
                id: client.id,
                mac: client.mac,
                active,
                credits: client.credits,
            },
            (mac, state),
        ))
    })
    .throttle(Duration::from_millis(250));

    let mut headers = HeaderMap::new();
    headers.append("X-Accel-Buffering", "no".parse().unwrap()); // NOTE: Must be set if nginx is used as reverse-proxy

    let stream = Sse::new(
        stream
            .map(|data| Event::default().json_data(data).unwrap())
            .map(Ok),
    )
    .keep_alive(KeepAlive::default());
    (headers, stream)
}

// NOTE: Below are local (debugging) implementation of the endpoints

#[cfg(debug_assertions)]
pub async fn data() -> Result<Json<ClientData>, StatusCode> {
    Ok(Json(ClientData {
        id: 1,
        mac: "20:16:d8:15:b1:26".to_owned(),
        credits: 0,
        remaining_seconds: 123121,
        active: false,
    }))
}

#[cfg(debug_assertions)]
pub async fn data_stream() -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let mut remaining_seconds = 123121;

    let stream = stream::repeat_with(move || {
        remaining_seconds -= 1;

        Event::default()
            .json_data(ClientData {
                id: 1,
                mac: "20:16:d8:15:b1:26".to_owned(),
                credits: 0,
                remaining_seconds,
                active: false,
            })
            .unwrap()
    })
    .map(Ok)
    .throttle(Duration::from_millis(1000));

    Sse::new(stream).keep_alive(KeepAlive::default())
}
