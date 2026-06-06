use crate::events::{AppEvent, BluetoothEvent};
use anyhow::Result;
use tokio::sync::mpsc;

pub async fn scan_devices(tx: mpsc::Sender<AppEvent>) -> Result<()> {
    // Placeholder: simulates periodic Bluetooth adapter state updates
    // In production, this would listen to BlueZ DBus ObjectManager signals
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

#[cfg(test)]
mod tests {
    #[test]
    fn test_scan_placeholder() {
        // Placeholder test
        assert!(true);
    }
}
