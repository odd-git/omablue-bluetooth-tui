use crate::events::{AppEvent, BluetoothEvent};
use anyhow::Result;
use tokio::sync::mpsc;

pub async fn scan_devices(tx: mpsc::Sender<AppEvent>) -> Result<()> {
    let connection = zbus::Connection::system().await?;

    // For now, just keep adapter updated
    // Full ObjectManager integration requires proper DBus path handling
    loop {
        if let Ok(powered) = super::adapter::is_powered().await {
            let _ = tx
                .send(AppEvent::BluetoothEvent(BluetoothEvent::AdapterPowered(
                    powered,
                )))
                .await;
        }

        tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    }
}

fn extract_mac_from_path(path: &str) -> String {
    // Path format: /org/bluez/hci0/dev_XX_XX_XX_XX_XX_XX
    if let Some(dev_part) = path.split("dev_").last() {
        dev_part.replace('_', ":")
    } else {
        String::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_mac_from_path() {
        let path = "/org/bluez/hci0/dev_1A_2B_3C_4D_5E_6F";
        assert_eq!(extract_mac_from_path(path), "1A:2B:3C:4D:5E:6F");
    }
}
