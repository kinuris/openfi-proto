#![cfg(test)]

use std::collections::HashMap;

use tokio::sync::Mutex;

use crate::{ClientConnections, RawClientDataCollection};

#[tokio::test]
async fn test_auth_client_state() {
    let data = "{
        \"client_list_length\":\"2\",
        \"clients\":{
          \"20:16:d8:15:b1:26\":{
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
          },
          \"7e:c6:be:e4:b4:97\":{
            \"gatewayname\":\"openNDS%20Node%3a7cc2c6480d3b%20\",
            \"gatewayaddress\":\"192.168.1.1:2050\",
            \"gatewayfqdn\":\"status.client\",
            \"version\":\"10.1.1\",
            \"client_type\":\"cpd_can\",
            \"mac\":\"7e:c6:be:e4:b4:97\",
            \"ip\":\"192.168.1.195\",
            \"clientif\":\"enx7cc2c6480d3b\",
            \"session_start\":\"0\",
            \"session_end\":\"null\",
            \"last_active\":\"1691111942\",
            \"token\":\"bace32b1\",
            \"state\":\"Preauthenticated\",
            \"custom\":\"none\",
            \"download_rate_limit_threshold\":\"null\",
            \"download_packet_rate\":\"null\",
            \"download_bucket_size\":\"null\",
            \"upload_rate_limit_threshold\":\"null\",
            \"upload_packet_rate\":\"null\",
            \"upload_bucket_size\":\"null\",
            \"download_quota\":\"null\",
            \"upload_quota\":\"null\",
            \"download_this_session\":\"0\",
            \"download_session_avg\":\"0.00\",
            \"upload_this_session\":\"0\",
            \"upload_session_avg\":\"0.00\"
          }
        }
      }";

    let users: ClientConnections = std::sync::Arc::new(Mutex::new(HashMap::from([
        ("7e:c6:be:e4:b4:97".to_owned(), 123321),
        ("6d:c6:hf:e4:24:88".to_owned(), 82731),
        ("20:16:d8:15:b1:26".to_owned(), 28398),
    ])));

    // NOTE: Simplified function logic of lib.rs/assure_auth_client_state
    let output = serde_json::from_str::<RawClientDataCollection>(
        std::str::from_utf8(data.as_bytes()).unwrap(),
    )
    .unwrap();

    let mut users = users.lock().await;

    // NOTE: Removes users in 'users' if the mac (AKA. key) is not found in 'output.clients'
    users.retain(|mac, _| {
        if !output.clients.contains_key(mac) {
            return false;
        }

        return true;
    });

    assert!(
        users.contains_key("20:16:d8:15:b1:26")
            && *users.get("20:16:d8:15:b1:26").unwrap() == 28398
    );
    assert!(
        users.contains_key("7e:c6:be:e4:b4:97")
            && *users.get("7e:c6:be:e4:b4:97").unwrap() == 123321
    );
    assert!(!users.contains_key("6d:c6:hf:e4:24:88"));

    // NOTE: Removes users in 'users' that are not 'state: Authenticated'
    users.retain(|mac, _| {
        let value = output.clients.get(mac).unwrap();

        value.state == "Authenticated".to_owned()
    });

    assert!(
        users.contains_key("20:16:d8:15:b1:26")
            && *users.get("20:16:d8:15:b1:26").unwrap() == 28398
    );
    assert!(!users.contains_key("7e:c6:be:e4:b4:97"));
    assert!(!users.contains_key("6d:c6:hf:e4:24:88"));
}
