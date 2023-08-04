#![cfg(test)]
use std::collections::HashMap;

use crate::RawClientData;
use crate::RawClientDataCollection;

#[test]
fn parsing_single_client_only() {
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

    assert_eq!(
        data,
        RawClientData {
            mac: "20:16:d8:15:b1:26".to_owned(),
            ip: "192.168.1.131".to_owned(),
            state: "Authenticated".to_owned(),
            clientif: "enx7cc2c6480d3bend0".to_owned(),
            last_active: "1691113598".to_owned(),
            download_quota: Some("null".to_owned()),
            upload_quota: Some("null".to_owned()),
            session_start: "1691112539".to_owned(),
            session_end: "1691198939".to_owned()
        }
    );
}

#[test]
fn parsing_all_clients() {
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

    let data = serde_json::from_str::<RawClientDataCollection>(data).unwrap();

    assert_eq!(
        data,
        RawClientDataCollection {
            client_list_length: "2".to_owned(),
            clients: HashMap::from([
                (
                    "20:16:d8:15:b1:26".to_owned(),
                    RawClientData {
                        mac: "20:16:d8:15:b1:26".to_owned(),
                        ip: "192.168.1.131".to_owned(),
                        state: "Authenticated".to_owned(),
                        clientif: "enx7cc2c6480d3bend0".to_owned(),
                        last_active: "1691113598".to_owned(),
                        download_quota: Some("null".to_owned()),
                        upload_quota: Some("null".to_owned()),
                        session_start: "1691112539".to_owned(),
                        session_end: "1691198939".to_owned()
                    }
                ),
                (
                    "7e:c6:be:e4:b4:97".to_owned(),
                    RawClientData {
                        mac: "7e:c6:be:e4:b4:97".to_owned(),
                        ip: "192.168.1.195".to_owned(),
                        state: "Preauthenticated".to_owned(),
                        clientif: "enx7cc2c6480d3b".to_owned(),
                        last_active: "1691111942".to_owned(),
                        download_quota: Some("null".to_owned()),
                        upload_quota: Some("null".to_owned()),
                        session_start: "0".to_owned(),
                        session_end: "null".to_owned()
                    }
                )
            ]),
        }
    )
}
