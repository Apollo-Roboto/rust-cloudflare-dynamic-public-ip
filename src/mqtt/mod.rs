#![allow(dead_code)]

use std::net::Ipv4Addr;

use bincode::ErrorKind;
use bytes::Bytes;
use log::debug;
use rumqttc::v5::mqttbytes::QoS;
use rumqttc::v5::{AsyncClient, ClientError, MqttOptions};
use rumqttc::NetworkOptions;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::thread;
use tokio::task;

pub struct MqttClient {
    client: AsyncClient,
    base_topic: String,
}

impl MqttClient {
    pub async fn new(host: &str, port: u16, id: &str, base_topic: &str) -> Self {
        let mut mqttoptions = MqttOptions::new(id, host, port);
        mqttoptions.set_keep_alive(std::time::Duration::from_secs(5));

        let (client, mut _eventloop) = AsyncClient::new(mqttoptions, 10);

        // todo: configure the base topic name
        // todo: configure the authentication

        Self {
            client,
            base_topic: String::from(base_topic),
        }
    }

    pub async fn publish_ip_change(&self, payload: IpChangeMessage) -> Result<(), ClientError> {
        let topic = format!("{}/ipchange", self.base_topic);
        debug!("MQTT publishing to {}", topic);
        self.client
            .publish(&topic, QoS::ExactlyOnce, false, payload)
            .await
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IpChangeMessage {
    pub old: Ipv4Addr,
    pub new: Ipv4Addr,
}

// impl From<&IpChangeMessage> for Vec<u8> {
//     fn from(value: &IpChangeMessage) -> Self {
//         bincode::serialize(value).unwrap()
//     }
// }

// impl From<IpChangeMessage> for Vec<u8> {
//     fn from(value: IpChangeMessage) -> Self {
//         bincode::serialize(&value).unwrap()
//     }
// }

// impl TryFrom<&[u8]> for IpChangeMessage {
//     type Error = Box<ErrorKind>;

//     fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
//         bincode::deserialize(value)
//     }
// }

impl Into<Bytes> for IpChangeMessage {
    fn into(self) -> Bytes {
        let json = serde_json::to_vec(&self).expect("Failed to serialize IpChangeMessage to JSON");
        Bytes::from(json)
    }
}
