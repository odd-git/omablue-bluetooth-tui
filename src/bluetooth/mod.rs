pub mod adapter;
pub mod devices;
pub mod scanner;

use crate::app::BluetoothAction;
use crate::events::AppEvent;
use anyhow::Result;
use tokio::sync::mpsc;

pub async fn run(tx: mpsc::Sender<AppEvent>) {
    if let Err(e) = scanner::scan_devices(tx.clone()).await {
        tracing::error!("Bluetooth scanner error: {}", e);
        let _ = tx
            .send(AppEvent::BluetoothEvent(crate::events::BluetoothEvent::Error(
                format!("Scanner error: {}", e),
            )))
            .await;
    }
}

pub async fn dispatch(action: &BluetoothAction) -> Result<()> {
    match action {
        BluetoothAction::Connect(address) => {
            devices::connect_device(address).await?;
        }
        BluetoothAction::Disconnect(address) => {
            devices::disconnect_device(address).await?;
        }
        BluetoothAction::TogglePower => {
            adapter::toggle_power().await?;
        }
        BluetoothAction::StartScan => {
            adapter::start_discovery().await?;
        }
        BluetoothAction::StopScan => {
            adapter::stop_discovery().await?;
        }
        BluetoothAction::SetTrusted(address) => {
            devices::set_trusted(address, true).await?;
        }
        BluetoothAction::Pair => {
            // Pair will be handled by scanner discovering new devices
        }
    }
    Ok(())
}
