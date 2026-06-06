use anyhow::{anyhow, Result};
use std::process::Command;

fn run_bluetoothctl(args: &[&str]) -> Result<String> {
    let output = Command::new("bluetoothctl")
        .args(args)
        .output()?;

    if !output.status.success() {
        return Err(anyhow!(
            "bluetoothctl failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    Ok(String::from_utf8(output.stdout)?)
}

pub async fn is_powered() -> Result<bool> {
    match run_bluetoothctl(&["show"]) {
        Ok(output) => {
            let powered = output.contains("Powered: yes");
            tracing::debug!("Adapter powered: {}", powered);
            Ok(powered)
        }
        Err(e) => {
            tracing::warn!("Failed to get Powered state: {}", e);
            Err(anyhow!("BlueZ Adapter not found"))
        }
    }
}

pub async fn start_discovery() -> Result<()> {
    match run_bluetoothctl(&["scan", "on"]) {
        Ok(_) => {
            tracing::info!("Discovery started");
            Ok(())
        }
        Err(e) => {
            tracing::error!("Failed to start discovery: {}", e);
            Err(e)
        }
    }
}

pub async fn stop_discovery() -> Result<()> {
    match run_bluetoothctl(&["scan", "off"]) {
        Ok(_) => {
            tracing::info!("Discovery stopped");
            Ok(())
        }
        Err(e) => {
            tracing::error!("Failed to stop discovery: {}", e);
            Err(e)
        }
    }
}

pub async fn toggle_power() -> Result<()> {
    let current = is_powered().await.unwrap_or(false);
    set_powered(!current).await
}

pub async fn set_powered(powered: bool) -> Result<()> {
    let arg = if powered { "on" } else { "off" };
    match run_bluetoothctl(&["power", arg]) {
        Ok(_) => {
            tracing::info!("Adapter power set to: {}", powered);
            Ok(())
        }
        Err(e) => {
            tracing::error!("Failed to set power: {}", e);
            Err(e)
        }
    }
}
