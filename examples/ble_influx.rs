//! Discover Bluetooth devices and list them.

use bluer::{Adapter, AdapterEvent, Address, DeviceEvent};
use futures::{pin_mut, stream::SelectAll, StreamExt};
use influxdb::{Client, InfluxDbWriteable};
use log::{error, LevelFilter};
use std::io::Write;
use std::{collections::HashSet, env};

async fn query_device(
    adapter: &Adapter,
    addr: Address,
    influxclient: &Client,
) -> bluer::Result<()> {
    let device = adapter.device(addr)?;

    // Implement Error matching
    // Error: Error { kind: Internal(DBus("org.freedesktop.DBus.Error.NoReply")), message: "Message recipient disconnected from message bus without replying" }
    let manufacture_data = device.manufacturer_data().await?;
    match manufacture_data {
        None => {}
        Some(payload) => {
            // Match iBS03 series
            match payload.get(&0x000d) {
                None => {}
                Some(payload) => match ingics::parse_data(payload) {
                    None => error!("Failed to parse payload"),
                    Some(i) => {
                        //println!("{:?}", i);
                        let write_result = influxclient.query(i.into_query("sensor")).await;
                        if !write_result.is_ok() {
                            error!("Failed to write result to InfluxDB");
                        }
                    }
                },
            }
            // Match iBS05 series
            match payload.get(&0x082c) {
                None => {}
                Some(payload) => match ingics::parse_data(payload) {
                    None => error!("Failed to parse payload"),
                    Some(i) => {
                        //println!("{:?}", i);
                        let write_result = influxclient.query(i.into_query("sensor")).await;
                        if !write_result.is_ok() {
                            error!("Failed to write result to InfluxDB");
                        }
                    }
                },
            }
        }
    }
    Ok(())
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> bluer::Result<()> {
    // Setup main logger
    match std::env::var("RUST_LOG_STYLE") {
        Ok(s) if s == "SYSTEMD" => env_logger::builder()
            .format(|buf, record| {
                writeln!(
                    buf,
                    "<{}>{}: {}",
                    match record.level() {
                        log::Level::Error => 3,
                        log::Level::Warn => 4,
                        log::Level::Info => 6,
                        log::Level::Debug => 7,
                        log::Level::Trace => 7,
                    },
                    record.target(),
                    record.args()
                )
            })
            .filter(None, LevelFilter::Info)
            .parse_default_env()
            .init(),
        _ => env_logger::builder()
            .filter_level(LevelFilter::Info)
            .parse_default_env()
            .init(),
    };

    let filter_addr: HashSet<_> = env::args()
        .filter_map(|arg| arg.parse::<Address>().ok())
        .collect();

    let session = bluer::Session::new().await?;
    let adapter = session.default_adapter().await?;
    println!(
        "Querying devices using Bluetooth adapater {}\n",
        adapter.name()
    );
    adapter.set_powered(true).await?;

    let device_events = adapter.discover_devices().await?;
    pin_mut!(device_events);

    let influxclient = Client::new("http://localhost:8086", "db0");

    let mut all_change_events = SelectAll::new();

    loop {
        tokio::select! {
            Some(device_event) = device_events.next() => {
                match device_event {
                    AdapterEvent::DeviceAdded(addr) => {
                        if !filter_addr.is_empty() && !filter_addr.contains(&addr) {
                            continue;
                        }

                        let res = query_device(&adapter, addr, &influxclient).await;

                        if let Err(err) = res {
                            println!("    Error: {}", &err);
                        }

                        let device = adapter.device(addr)?;
                        let change_events = device.events().await?.map(move |evt| (addr, evt));
                        all_change_events.push(change_events);
                    }
                    _ => (),
                }
                println!();
            }
            Some((addr, DeviceEvent::PropertyChanged(_property))) = all_change_events.next() => {
                let res = query_device(&adapter, addr, &influxclient).await;

                if let Err(err) = res {
                    println!("    Error: {}", &err);
                }
            }

            else => break
        }
    }

    Ok(())
}
