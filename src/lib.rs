pub mod extensions;
pub mod handlers;
pub mod params;
pub mod responders;

#[derive(serde::Deserialize, serde::Serialize, Debug)]
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

pub mod client_timer {
    use std::{process::Command, time::Duration};

    use entity::client;
    use sea_orm::{
        ActiveModelTrait, ActiveValue, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter,
    };
    use tokio::time::interval;

    use crate::ClientConnections;

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

                let mut client: client::ActiveModel = client.into();

                client.remaining_seconds = ActiveValue::Set(*time - 1);
                client.update(&db).await.unwrap();

                *time -= 1;

                if *time <= 0 {
                    Command::new("ndsctl")
                        .arg("deauth")
                        .arg(&mac)
                        .spawn()
                        .unwrap();
                }
            }

            active_map.retain(|_, v| *v > 0);
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
