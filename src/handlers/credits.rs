#![allow(warnings, unused)]
use axum::{
    extract::{Path, State},
    http::StatusCode,
};
use axum_sessions::extractors::ReadableSession;
use entity::{client, plan};
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, EntityTrait, IntoActiveModel, QueryFilter,
};

use crate::AppState;

#[cfg(not(debug_assertions))]
pub async fn spend_credits(
    State(state): State<AppState>,
    Path((kind, id)): Path<(String, i32)>,
    session: ReadableSession,
) -> Result<(), (StatusCode, String)> {
    let mac = session.get::<String>("mac").unwrap();

    let client = client::Entity::find()
        .filter(client::Column::Mac.contains(&mac))
        .one(&state.connection)
        .await
        .unwrap()
        .unwrap();

    // TODO: check if kind == "TIME" (entity::plan) / "DATA" (entity::data_plan)
    // NOTE: This is different from redeemable_codes "TIME", "CREDIT" and "DATA"

    if kind != "TIME" {
        todo!()
    }

    let plan = plan::Entity::find_by_id(id)
        .one(&state.connection)
        .await
        .unwrap()
        .unwrap();

    let credits = client.credits - plan.credit_cost;

    if credits < 0 {
        return Err((StatusCode::FORBIDDEN, "Not enough credits".to_owned()));
    }

    let remaining_seconds = client.remaining_seconds;

    let mut active_client = client.into_active_model();

    active_client.remaining_seconds = ActiveValue::Set(remaining_seconds + plan.seconds_given);
    active_client.credits = ActiveValue::Set(credits);

    active_client.update(&state.connection).await.unwrap();

    Ok(())
}

#[cfg(debug_assertions)]
pub async fn spend_credits() -> Result<(), (StatusCode, String)> {
    Ok(())
}
