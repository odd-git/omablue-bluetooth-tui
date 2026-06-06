use anyhow::{anyhow, Result};
use zbus::Connection;

const ADAPTER_PATH: &str = "/org/bluez/hci0";
const ADAPTER_INTERFACE: &str = "org.bluez.Adapter1";

async fn call_adapter_method(method: &str) -> Result<()> {
    let connection = Connection::system().await?;

    connection
        .call_method(
            Some("org.bluez"),
            ADAPTER_PATH,
            Some(ADAPTER_INTERFACE),
            method,
            &(),
        )
        .await?;

    Ok(())
}

async fn get_adapter_property(prop: &str) -> Result<bool> {
    let connection = Connection::system().await?;

    let result: bool = connection
        .call_method(
            Some("org.freedesktop.DBus.Properties"),
            ADAPTER_PATH,
            Some("org.freedesktop.DBus.Properties"),
            "Get",
            &(ADAPTER_INTERFACE, prop),
        )
        .await?;

    Ok(result)
}

pub async fn is_powered() -> Result<bool> {
    match get_adapter_property("Powered").await {
        Ok(powered) => {
            tracing::debug!("Adapter powered: {}", powered);
            Ok(powered)
        }
        Err(e) => {
            tracing::warn!("Failed to get Powered property: {}", e);
            Err(anyhow!("BlueZ Adapter not found"))
        }
    }
}

pub async fn start_discovery() -> Result<()> {
    match call_adapter_method("StartDiscovery").await {
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
    match call_adapter_method("StopDiscovery").await {
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
    let connection = Connection::system().await?;

    connection
        .call_method(
            Some("org.freedesktop.DBus.Properties"),
            ADAPTER_PATH,
            Some("org.freedesktop.DBus.Properties"),
            "Set",
            &(ADAPTER_INTERFACE, "Powered", powered),
        )
        .await?;

    tracing::info!("Adapter power set to: {}", powered);
    Ok(())
}
