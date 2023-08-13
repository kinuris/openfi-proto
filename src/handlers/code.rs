#![allow(warnings, unused)]
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use axum_sessions::extractors::ReadableSession;
use entity::{client, redeemable_code};
use rand::{distributions::Alphanumeric, Rng};
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, EntityTrait, IntoActiveModel, QueryFilter,
};

use crate::{AppState, CodeData};

// TODO: gen_code should return 'CodeData'
#[cfg(not(debug_assertions))]
pub async fn gen_code(
    session: ReadableSession,
    Path((kind, units)): Path<(String, i32)>,
    State(state): State<AppState>,
) -> Result<String, (StatusCode, String)> {
    let admin = session.get::<bool>("admin").is_some();

    if !admin {
        return Err((
            StatusCode::FORBIDDEN,
            "You are not an admin, this incident will be reported".to_owned(),
        ));
    }

    let in_code = loop {
        let code = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(10)
            .map(char::from)
            .map(|c| c.to_ascii_uppercase())
            .collect::<String>();

        let exists = redeemable_code::Entity::find()
            .filter(redeemable_code::Column::Code.contains(&code))
            .one(&state.connection)
            .await
            .unwrap()
            .is_some();

        if !exists {
            break code;
        }
    };

    let redeemable_code = redeemable_code::ActiveModel {
        code: ActiveValue::Set(in_code.clone().into()),
        kind: ActiveValue::Set(kind),
        units: ActiveValue::Set(units),
        ..Default::default()
    };

    redeemable_code.insert(&state.connection).await.unwrap();

    Ok(in_code)
}

#[cfg(not(debug_assertions))]
pub async fn get_codes(
    session: ReadableSession,
    State(state): State<AppState>,
) -> Result<Json<Vec<CodeData>>, (StatusCode, String)> {
    let admin = session.get::<bool>("admin").is_some();

    if !admin {
        return Err((
            StatusCode::FORBIDDEN,
            "You are not an admin, this incident will be reported".to_owned(),
        ));
    }

    let codes = redeemable_code::Entity::find()
        .all(&state.connection)
        .await
        .unwrap()
        .into_iter()
        .map(|raw| CodeData {
            id: raw.id,
            code: raw
                .code
                .into_iter()
                .map(|c| c.to_ascii_uppercase())
                .map(char::from)
                .collect(),
            kind: raw.kind,
            units: raw.units,
        })
        .collect::<Vec<_>>();

    Ok(Json(codes))
}

#[derive(serde::Deserialize)]
pub struct CodeJSONBody {
    code: String,
}

#[cfg(not(debug_assertions))]
pub async fn redeem_code(
    session: ReadableSession,
    State(state): State<AppState>,
    Json(body): Json<CodeJSONBody>,
) -> Result<(), StatusCode> {
    let code = redeemable_code::Entity::find()
        .filter(redeemable_code::Column::Code.contains(body.code))
        .one(&state.connection)
        .await
        .unwrap();

    if code.is_none() {
        return Err(StatusCode::NOT_FOUND);
    }

    // TODO: Split logic depeding on code.kind, for now only time codes are redeemable

    let client = client::Entity::find()
        .filter(client::Column::Mac.contains(session.get::<String>("mac").unwrap()))
        .one(&state.connection)
        .await
        .unwrap()
        .unwrap();

    let total_seconds = client.remaining_seconds + code.as_ref().unwrap().units;

    let mut client = client.into_active_model();
    client.remaining_seconds = ActiveValue::Set(total_seconds);
    client.update(&state.connection).await.unwrap();

    code.unwrap()
        .into_active_model()
        .delete(&state.connection)
        .await
        .unwrap();

    Ok(())
}

#[cfg(debug_assertions)]
pub async fn gen_code() -> Result<String, (StatusCode, String)> {
    let code = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(10)
        .map(char::from)
        .map(|c| c.to_ascii_uppercase())
        .collect::<String>();

    Ok(code)
}

#[cfg(debug_assertions)]
pub async fn get_codes(
    session: ReadableSession,
    State(state): State<AppState>,
) -> Result<Json<Vec<CodeData>>, (StatusCode, String)> {
    use crate::ClientData;

    let codes = (0..5)
        .map(|_| CodeData {
            id: rand::thread_rng().gen_range((0..=10000)),
            code: rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .take(10)
                .map(|c| c.to_ascii_uppercase())
                .map(char::from)
                .collect(),
            units: rand::thread_rng().gen_range((1000..=100000)),
            kind: if rand::thread_rng().gen_bool(0.5) {
                "TIME".to_owned()
            } else {
                "DATA".to_owned()
            },
        })
        .collect::<Vec<_>>();

    Ok(Json(codes))
}

#[cfg(debug_assertions)]
pub async fn redeem_code(Json(body): Json<CodeJSONBody>) -> Result<(), StatusCode> {
    println!("{}", body.code);

    Ok(())
}
