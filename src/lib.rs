pub mod extensions;
pub mod handlers;
pub mod params;
pub mod responders;
pub mod test;

#[derive(serde::Deserialize, serde::Serialize, Debug, PartialEq)]
pub struct RawClientData {
    pub mac: String,
    pub ip: String,
    pub state: String,
    pub clientif: String,
    pub session_start: String,
    pub session_end: String,
    pub last_active: String,
    pub download_quota: Option<String>,
    pub upload_quota: Option<String>,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, PartialEq)]
pub struct RawClientDataCollection {
    client_list_length: String,
    clients: HashMap<String, RawClientData>,
}

pub mod client_timer {
    use std::{process::Command, time::Duration};

    use entity::client;
    use sea_orm::{
        ActiveModelTrait, ActiveValue, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter,
    };
    use tokio::time::interval;

    use crate::{ClientConnections, RawClientData, RawClientDataCollection};

    pub async fn decrement_active_user(db: DatabaseConnection, users: ClientConnections) {
        let mut interval = interval(Duration::from_secs(1));
        loop {
            interval.tick().await;

            let mut active_map = users.lock().await;

            for (mac, time) in active_map.iter_mut() {
                let client = client::Entity::find()
                    .filter(client::Column::Mac.contains(mac))
                    .one(&db)
                    .await
                    .unwrap()
                    .unwrap();

                let remaining_seconds = client.remaining_seconds - 1;
                let mut client: client::ActiveModel = client.into();

                client.remaining_seconds = ActiveValue::Set(remaining_seconds);
                client.update(&db).await.unwrap();

                *time = remaining_seconds;

                if *time <= 0 {
                    Command::new("ndsctl")
                        .arg("deauth")
                        .arg(mac)
                        .spawn()
                        .unwrap()
                        .wait()
                        .unwrap();
                }
            }

            active_map.retain(|_, v| *v > 0);
        }
    }

    pub async fn assure_auth_client_state(users: ClientConnections) {
        let mut interval = interval(Duration::from_secs(60));

        loop {
            interval.tick().await;

            let output = Command::new("ndsctl").arg("json").output().unwrap();
            let output = serde_json::from_str::<RawClientDataCollection>(
                std::str::from_utf8(&output.stdout).unwrap(),
            )
            .unwrap(); // TODO: Solve panics at runtime

            let mut users = users.lock().await;

            // TODO: Replace stupid and potentially cyclical logic
            for (mac, RawClientData { state, .. }) in &output.clients {
                if state == "Authenticated" && !users.contains_key(mac) {
                    Command::new("ndsctl")
                        .arg("deauth")
                        .arg(mac)
                        .spawn()
                        .unwrap()
                        .wait()
                        .unwrap();
                }
            }

            users.retain(|mac, _| {
                if !output.clients.contains_key(mac) {
                    Command::new("ndsctl")
                        .arg("deauth")
                        .arg(mac)
                        .spawn()
                        .unwrap()
                        .wait()
                        .unwrap();

                    return false;
                }

                return true;
            });

            users.retain(|mac, _| {
                let value = output.clients.get(mac).unwrap();

                value.state == "Authenticated".to_owned()
            });
        }
    }
}

use sea_orm::DatabaseConnection;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;

pub type ClientConnections = Arc<Mutex<HashMap<String, i32>>>;

#[derive(Clone)]
pub struct AppState {
    pub connection: DatabaseConnection,
    pub active_clients: ClientConnections,
}

#[derive(serde::Serialize)]
pub struct ClientData {
    pub id: i32,
    pub mac: String,
    pub credits: i32,
    pub remaining_seconds: i32,
    pub active: bool,
}

#[derive(serde::Serialize)]
pub struct CodeData {
    pub id: i32,
    pub code: String,
    pub kind: String,
    pub units: i32,
}

#[derive(serde::Serialize)]
pub struct PlanCollectionData {
    pub time_plans: Vec<PlanData>,
    pub data_plans: Vec<DataPlanData>,
}

#[derive(serde::Serialize)]
pub struct PlanData {
    pub id: i32,
    pub name: String,
    pub credit_cost: i32,
    pub seconds_given: i32,
}

impl From<entity::plan::Model> for PlanData {
    fn from(value: entity::plan::Model) -> Self {
        Self {
            id: value.id,
            name: value.name,
            credit_cost: value.credit_cost,
            seconds_given: value.seconds_given,
        }
    }
}

#[derive(serde::Serialize)]
pub struct DataPlanData {
    pub id: i32,
    pub name: String,
    pub credit_cost: i32,
    pub megabytes_given: i32,
}

impl From<entity::data_plan::Model> for DataPlanData {
    fn from(value: entity::data_plan::Model) -> Self {
        Self {
            id: value.id,
            name: value.name,
            credit_cost: value.credit_cost,
            megabytes_given: value.megabytes_given,
        }
    }
}
