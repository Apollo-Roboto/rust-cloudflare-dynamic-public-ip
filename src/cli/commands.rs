use std::{net::Ipv4Addr, sync::mpsc};

use clap::Args;
use log::{debug, error, info, trace, warn};

use crate::cloudflare::{
    client::{CloudFlareClient, CloudFlareClientError},
    models::UpdateDNSRecordRequest,
};

#[derive(Debug, Args)]
pub struct CurrentArguments {}

pub async fn current_command(_args: &CurrentArguments) -> i32 {
    match public_ip::addr_v4().await {
        Some(ip) => {
            info!("{}", ip);
            0
        }
        None => {
            error!("Could not get public IP");
            1
        }
    }
}

#[derive(Debug, Args)]
pub struct MonitorArguments {}

pub async fn monitor_command(_args: &MonitorArguments) -> i32 {
    let cloudflare_token = std::env::var("CLOUDFLARE_TOKEN")
        .expect("Environment variable CLOUDFLARE_TOKEN is not set");
    let cloudflare_zone_id = std::env::var("CLOUDFLARE_ZONE_ID")
        .expect("Environment variable CLOUDFLARE_ZONE_ID is not set");

    let cloudflare_client = CloudFlareClient::new(&cloudflare_token, &cloudflare_zone_id);

    // let monitor_loop = MonitorLoop::new(std::time::Duration::from_secs(300));
    let monitor_loop = MonitorLoop::new(std::time::Duration::from_secs(5));

    monitor_loop.start();

    for message in monitor_loop.listen() {
        match message {
            MonitorLoopMessage::IpChanged { old_ip, new_ip } => {
                info!("IP address change detected from {} to {}", old_ip, new_ip);

                match update_ip(&cloudflare_client, old_ip, new_ip).await {
                    Ok(_) => {
                        info!("Successfully updated IP to {}", new_ip)
                    }
                    Err(e) => {
                        error!("Failed to update IP: {:?}", e)
                    }
                }
            }
            MonitorLoopMessage::CouldNotGetIp => warn!("Could not get public IP"),
            MonitorLoopMessage::NoChange => trace!("No IP change"),
        }
    }

    0
}

async fn update_ip(
    client: &CloudFlareClient,
    old_ip: Ipv4Addr,
    new_ip: Ipv4Addr,
) -> Result<(), CloudFlareClientError> {
    let records = match client
        .get_dns_records_with_content(&old_ip.to_string())
        .await
    {
        Ok(r) => r.result,
        Err(e) => return Err(e),
    };

    debug!("Found {} records to update", records.len());

    for record in records {
        let record_name = record.name.clone();
        debug!("Updating record {}", record_name);

        let mut new_record = UpdateDNSRecordRequest::from(record);
        new_record.content = new_ip.to_string();

        if let Err(e) = client.set_dns_record(new_record).await {
            error!("Failed to update record {}", record_name);
            return Err(e);
        }

        info!("Successfully updated record {}", record_name);
    }

    Ok(())
}

#[derive(Debug)]
enum MonitorLoopMessage {
    IpChanged { old_ip: Ipv4Addr, new_ip: Ipv4Addr },
    CouldNotGetIp,
    NoChange,
}

struct MonitorLoop {
    wait_time: std::time::Duration,
    tx: mpsc::Sender<MonitorLoopMessage>,
    rx: mpsc::Receiver<MonitorLoopMessage>,
}

impl MonitorLoop {
    fn new(wait_time: std::time::Duration) -> Self {
        let (tx, rx) = mpsc::channel();
        Self { wait_time, tx, rx }
    }

    fn start(&self) {
        let wait_time = self.wait_time;
        debug!("Loop wait time: {}ms", wait_time.as_millis());
        let tx = self.tx.clone();

        tokio::spawn(async move {
            let start_ip = public_ip::addr_v4()
                .await
                .expect("Could not get public IP address");

            info!("Current IP is {}", start_ip);

            let mut old_ip = start_ip;

            trace!("Starting loop");

            loop {
                if let Some(current_ip) = public_ip::addr_v4().await {
                    if old_ip != current_ip {
                        tx.send(MonitorLoopMessage::IpChanged {
                            old_ip,
                            new_ip: current_ip,
                        })
                        .unwrap();

                        old_ip = current_ip;
                    } else {
                        tx.send(MonitorLoopMessage::NoChange).unwrap();
                    }
                } else {
                    tx.send(MonitorLoopMessage::CouldNotGetIp).unwrap();
                }

                tokio::time::sleep(wait_time).await;
            }
        });
    }

    fn listen(&self) -> &mpsc::Receiver<MonitorLoopMessage> {
        &self.rx
    }
}
