use crossterm::event::{self, KeyEvent};
use tokio::sync::mpsc;

#[derive(Debug, Clone)]
pub enum AppEvent {
    Key(KeyEvent),
    BluetoothEvent(BluetoothEvent),
    ThemeChanged(crate::ui::theme::ThemeColors),
    Tick,
    Quit,
}

#[derive(Debug, Clone)]
pub enum BluetoothEvent {
    AdapterPowered(bool),
    DeviceDiscovered { address: String, name: String },
    DeviceConnected(String),
    DeviceDisconnected(String),
    DeviceRemoved(String),
    ScanStarted,
    ScanStopped,
    Error(String),
}

pub async fn read_keys(tx: mpsc::Sender<AppEvent>) {
    loop {
        if let Ok(true) = event::poll(std::time::Duration::from_millis(100)) {
            if let Ok(event::Event::Key(key)) = event::read() {
                let _ = tx.send(AppEvent::Key(key)).await;
            }
        }
        let _ = tx.send(AppEvent::Tick).await;
    }
}
