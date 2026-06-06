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

pub struct Device1Proxy {
    pub address: String,
    pub name: String,
}

pub async fn get_device_proxy(address: &str) -> Result<Device1Proxy> {
    match run_bluetoothctl(&["info", address]) {
        Ok(output) => {
            let name = output
                .lines()
                .find(|l| l.contains("Name:"))
                .and_then(|l| l.split_whitespace().last())
                .unwrap_or(&address)
                .to_string();

            Ok(Device1Proxy {
                address: address.to_string(),
                name,
            })
        }
        Err(_) => Ok(Device1Proxy {
            address: address.to_string(),
            name: format!("Device-{}", address),
        }),
    }
}

pub async fn connect_device(address: &str) -> Result<()> {
    match run_bluetoothctl(&["connect", address]) {
        Ok(_) => {
            tracing::info!("Connecting to device: {}", address);
            Ok(())
        }
        Err(e) => {
            tracing::error!("Failed to connect to {}: {}", address, e);
            Err(e)
        }
    }
}

pub async fn disconnect_device(address: &str) -> Result<()> {
    match run_bluetoothctl(&["disconnect", address]) {
        Ok(_) => {
            tracing::info!("Disconnecting from device: {}", address);
            Ok(())
        }
        Err(e) => {
            tracing::error!("Failed to disconnect from {}: {}", address, e);
            Err(e)
        }
    }
}

pub async fn pair_device(address: &str) -> Result<()> {
    match run_bluetoothctl(&["pair", address]) {
        Ok(_) => {
            tracing::info!("Pairing with device: {}", address);
            Ok(())
        }
        Err(e) => {
            tracing::error!("Failed to pair with {}: {}", address, e);
            Err(e)
        }
    }
}

pub async fn set_trusted(address: &str, trusted: bool) -> Result<()> {
    let cmd = if trusted { "trust" } else { "untrust" };
    match run_bluetoothctl(&[cmd, address]) {
        Ok(_) => {
            tracing::info!("Set trusted={} for device: {}", trusted, address);
            Ok(())
        }
        Err(e) => {
            tracing::error!("Failed to set trusted: {}", e);
            Err(e)
        }
    }
}

pub async fn get_device_info(address: &str) -> Result<(String, bool, bool, bool)> {
    match run_bluetoothctl(&["info", address]) {
        Ok(output) => {
            let name = output
                .lines()
                .find(|l| l.contains("Name:"))
                .and_then(|l| l.split_whitespace().last())
                .unwrap_or(&address)
                .to_string();

            let connected = output.contains("Connected: yes");
            let paired = output.contains("Paired: yes");
            let trusted = output.contains("Trusted: yes");

            Ok((name, connected, paired, trusted))
        }
        Err(_) => Ok((address.to_string(), false, false, false)),
    }
}
