use crate::events::{AppEvent, BluetoothEvent};
use anyhow::Result;
use tokio::sync::mpsc;
use std::process::Command;

pub async fn scan_devices(tx: mpsc::Sender<AppEvent>) -> Result<()> {
    loop {
        // Update adapter state
        if let Ok(powered) = super::adapter::is_powered().await {
            let _ = tx
                .send(AppEvent::BluetoothEvent(BluetoothEvent::AdapterPowered(
                    powered,
                )))
                .await;
        }

        // Scan for paired devices
        if let Ok(output) = Command::new("bluetoothctl")
            .args(&["devices", "Paired"])
            .output()
        {
            if let Ok(stdout) = String::from_utf8(output.stdout) {
                for line in stdout.lines() {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() >= 3 && parts[0] == "Device" {
                        let address = parts[1];
                        let name = parts[2..].join(" ");
                        let _ = tx
                            .send(AppEvent::BluetoothEvent(
                                BluetoothEvent::DeviceDiscovered {
                                    address: address.to_string(),
                                    name,
                                },
                            ))
                            .await;
                    }
                }
            }
        }

        tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_scan_placeholder() {
        // Placeholder test
        assert!(true);
    }
}
