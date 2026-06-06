use crate::events::{AppEvent, BluetoothEvent};
use anyhow::Result;
use futures_util::StreamExt;
use tokio::sync::mpsc;
use zbus::fdo::ObjectManagerProxy;

pub async fn scan_devices(tx: mpsc::Sender<AppEvent>) -> Result<()> {
    let connection = zbus::Connection::system().await?;
    let object_manager = ObjectManagerProxy::new(&connection).await?;

    // Send initial adapter state
    if let Ok(powered) = super::adapter::is_powered().await {
        let _ = tx
            .send(AppEvent::BluetoothEvent(BluetoothEvent::AdapterPowered(
                powered,
            )))
            .await;
    }

    // Listen for new interfaces (devices)
    let mut interfaces_added = object_manager.receive_interfaces_added().await?;

    while let Some(signal) = interfaces_added.next().await {
        if let Ok(args) = signal.args() {
            let path = args.object_path();
            let interfaces = args.interfaces_added();

            // Check if this is a Device1 interface
            if let Some(_device_iface) = interfaces.get("org.bluez.Device1") {
                if let Ok(device) = super::devices::get_device_proxy(
                    &extract_mac_from_path(path.as_str()),
                ) {
                    if let Ok(name) = device.name().await {
                        let address = extract_mac_from_path(path.as_str());
                        let _ = tx
                            .send(AppEvent::BluetoothEvent(BluetoothEvent::DeviceDiscovered {
                                address,
                                name,
                            }))
                            .await;
                    }
                }
            }
        }
    }

    Ok(())
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
