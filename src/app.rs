use crate::events::{AppEvent, BluetoothEvent};
use crate::ui::theme::ThemeColors;
use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BluetoothDevice {
    pub address: String,
    pub name: String,
    pub connected: bool,
    pub paired: bool,
    pub trusted: bool,
    pub battery: Option<u8>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppMode {
    DeviceList,
    Scanning,
    ConfirmPair,
    ConfirmTrust,
    Error,
}

#[derive(Debug, Clone)]
pub enum ConfirmAction {
    Pair { address: String, name: String },
    Trust { address: String, name: String },
}

const SPINNER_FRAMES: &[&str] = &["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];

#[derive(Debug)]
pub struct AppState {
    pub mode: AppMode,
    pub devices: Vec<BluetoothDevice>,
    pub selected: usize,
    pub adapter_powered: bool,
    pub scanning: bool,
    pub theme: ThemeColors,
    pub status_msg: Option<String>,
    pub error_msg: Option<String>,
    pub quit: bool,
    pub confirm_action: Option<ConfirmAction>,
    pub spinner_frame: usize,
    pub operation_in_progress: bool,
    pub last_action: Option<String>,
}

impl AppState {
    pub async fn new() -> Result<Self> {
        let theme = ThemeColors::load().unwrap_or_else(|_| ThemeColors::fallback());

        Ok(Self {
            mode: AppMode::DeviceList,
            devices: Vec::new(),
            selected: 0,
            adapter_powered: false,
            scanning: false,
            theme,
            status_msg: Some("Loading Bluetooth devices...".to_string()),
            error_msg: None,
            quit: false,
            confirm_action: None,
            spinner_frame: 0,
            operation_in_progress: false,
            last_action: None,
        })
    }

    pub fn handle_key(&mut self, key: KeyEvent) -> Option<BluetoothAction> {
        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => {
                self.quit = true;
                None
            }
            KeyCode::Char('j') | KeyCode::Down => {
                if !self.devices.is_empty() {
                    self.selected = (self.selected + 1) % self.devices.len();
                }
                None
            }
            KeyCode::Char('k') | KeyCode::Up => {
                if !self.devices.is_empty() {
                    self.selected = if self.selected == 0 {
                        self.devices.len() - 1
                    } else {
                        self.selected - 1
                    };
                }
                None
            }
            KeyCode::Enter => {
                let device = self.devices.get(self.selected).cloned();
                if let Some(device) = device {
                    if device.connected {
                        self.start_operation("Disconnecting");
                        Some(BluetoothAction::Disconnect(device.address))
                    } else {
                        self.start_operation("Connecting");
                        Some(BluetoothAction::Connect(device.address))
                    }
                } else {
                    None
                }
            }
            KeyCode::Char('d') => {
                let device = self.devices.get(self.selected).cloned();
                if let Some(device) = device {
                    self.start_operation("Disconnecting");
                    Some(BluetoothAction::Disconnect(device.address))
                } else {
                    None
                }
            }
            KeyCode::Char('s') => {
                if self.scanning {
                    self.start_operation("Stopping scan");
                    Some(BluetoothAction::StopScan)
                } else {
                    self.start_operation("Scanning for devices");
                    Some(BluetoothAction::StartScan)
                }
            }
            KeyCode::Char('t') => {
                if key.modifiers.contains(KeyModifiers::SHIFT) {
                    // Shift+T = set trusted
                    let device = self.devices.get(self.selected).cloned();
                    if let Some(device) = device {
                        self.start_operation("Setting trusted");
                        Some(BluetoothAction::SetTrusted(device.address))
                    } else {
                        None
                    }
                } else {
                    // t = toggle adapter power
                    self.start_operation("Toggling Bluetooth");
                    Some(BluetoothAction::TogglePower)
                }
            }
            KeyCode::Char('p') => {
                self.start_operation("Pairing");
                Some(BluetoothAction::Pair)
            }
            KeyCode::Char('?') => {
                // Show help (would open overlay)
                None
            }
            _ => None,
        }
    }

    pub fn apply_bluetooth_event(&mut self, event: BluetoothEvent) {
        match event {
            BluetoothEvent::AdapterPowered(powered) => {
                self.adapter_powered = powered;
                self.status_msg = Some(format!("Bluetooth: {}", if powered { "ON" } else { "OFF" }));
            }
            BluetoothEvent::DeviceDiscovered { address, name } => {
                if !self.devices.iter().any(|d| d.address == address) {
                    self.devices.push(BluetoothDevice {
                        address,
                        name,
                        connected: false,
                        paired: false,
                        trusted: false,
                        battery: None,
                    });
                }
            }
            BluetoothEvent::DeviceConnected(address) => {
                if let Some(device) = self.devices.iter_mut().find(|d| d.address == address) {
                    device.connected = true;
                    self.status_msg = Some(format!("Connected: {}", device.name));
                    let _ = crate::notifications::notify_success(&device.name);
                }
            }
            BluetoothEvent::DeviceDisconnected(address) => {
                if let Some(device) = self.devices.iter_mut().find(|d| d.address == address) {
                    device.connected = false;
                    self.status_msg = Some(format!("Disconnected: {}", device.name));
                }
            }
            BluetoothEvent::DeviceRemoved(address) => {
                self.devices.retain(|d| d.address != address);
                if self.selected >= self.devices.len() && self.selected > 0 {
                    self.selected -= 1;
                }
            }
            BluetoothEvent::ScanStarted => {
                self.scanning = true;
                self.status_msg = Some("Scanning for Bluetooth devices...".to_string());
            }
            BluetoothEvent::ScanStopped => {
                self.scanning = false;
                self.status_msg = Some("Scan stopped".to_string());
            }
            BluetoothEvent::Error(msg) => {
                self.error_msg = Some(msg);
            }
        }
    }

    pub fn set_theme(&mut self, theme: ThemeColors) {
        self.theme = theme;
    }

    pub fn should_quit(&self) -> bool {
        self.quit
    }

    pub fn animate_spinner(&mut self) {
        if self.operation_in_progress {
            self.spinner_frame = (self.spinner_frame + 1) % SPINNER_FRAMES.len();
        }
    }

    pub fn get_spinner(&self) -> &'static str {
        if self.operation_in_progress {
            SPINNER_FRAMES[self.spinner_frame]
        } else {
            ""
        }
    }

    pub fn start_operation(&mut self, action: &str) {
        self.operation_in_progress = true;
        self.spinner_frame = 0;
        self.last_action = Some(action.to_string());
        self.status_msg = Some(format!("Running: {}...", action));
    }

    pub fn end_operation(&mut self, result: &str) {
        self.operation_in_progress = false;
        self.status_msg = Some(result.to_string());
    }
}

#[derive(Debug, Clone)]
pub enum BluetoothAction {
    Connect(String),
    Disconnect(String),
    Pair,
    SetTrusted(String),
    StartScan,
    StopScan,
    TogglePower,
}
