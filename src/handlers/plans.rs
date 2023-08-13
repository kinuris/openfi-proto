use axum::{extract::State, Json};
use entity::{data_plan, plan};
use sea_orm::EntityTrait;

use crate::{AppState, DataPlanData, PlanCollectionData, PlanData};

// SUGGESTION: Consider making this a Server-Sent Event
pub async fn plans(State(state): State<AppState>) -> Result<Json<PlanCollectionData>, ()> {
    let time_plans = plan::Entity::find()
        .all(&state.connection)
        .await
        .map_err(|_| ())?
        .into_iter()
        .map(|model| model.into())
        .collect::<Vec<PlanData>>();

    let data_plans = data_plan::Entity::find()
        .all(&state.connection)
        .await
        .map_err(|_| ())?
        .into_iter()
        .map(|model| model.into())
        .collect::<Vec<DataPlanData>>();

    Ok(Json(PlanCollectionData {
        data_plans,
        time_plans,
    }))
}
