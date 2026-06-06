use anyhow::{anyhow, Result};
use zbus::Connection;

const ADAPTER_PATH: &str = "/org/bluez/hci0";
const DEVICE_INTERFACE: &str = "org.bluez.Device1";

fn mac_to_path(address: &str) -> String {
    format!(
        "/org/bluez/hci0/dev_{}",
        address.replace(':', "_").to_uppercase()
    )
}

async fn call_device_method(address: &str, method: &str) -> Result<()> {
    let path = mac_to_path(address);
    let connection = Connection::system().await?;

    connection
        .call_method(
            Some("org.bluez"),
            &path,
            Some(DEVICE_INTERFACE),
            method,
            &(),
        )
        .await?;

    Ok(())
}

async fn get_device_property(address: &str, prop: &str) -> Result<String> {
    let path = mac_to_path(address);
    let connection = Connection::system().await?;

    let result: String = connection
        .call_method(
            Some("org.freedesktop.DBus.Properties"),
            &path,
            Some("org.freedesktop.DBus.Properties"),
            "Get",
            &(DEVICE_INTERFACE, prop),
        )
        .await?;

    Ok(result)
}

pub struct Device1Proxy {
    pub address: String,
    pub name: String,
}

pub async fn get_device_proxy(address: &str) -> Result<Device1Proxy> {
    match get_device_property(address, "Name").await {
        Ok(name) => Ok(Device1Proxy {
            address: address.to_string(),
            name,
        }),
        Err(e) => {
            tracing::warn!("Device {} not found: {}", address, e);
            Ok(Device1Proxy {
                address: address.to_string(),
                name: format!("Device-{}", address),
            })
        }
    }
}

pub async fn connect_device(address: &str) -> Result<()> {
    match call_device_method(address, "Connect").await {
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
    match call_device_method(address, "Disconnect").await {
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
    match call_device_method(address, "Pair").await {
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
    let path = mac_to_path(address);
    let connection = Connection::system().await?;

    connection
        .call_method(
            Some("org.freedesktop.DBus.Properties"),
            &path,
            Some("org.freedesktop.DBus.Properties"),
            "Set",
            &(DEVICE_INTERFACE, "Trusted", trusted),
        )
        .await?;

    tracing::info!("Set trusted={} for device: {}", trusted, address);
    Ok(())
}

pub async fn get_device_info(address: &str) -> Result<(String, bool, bool, bool)> {
    let device = get_device_proxy(address).await?;
    Ok((device.name, false, false, false))
}
