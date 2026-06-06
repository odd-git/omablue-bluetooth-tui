use anyhow::Result;

#[derive(Debug, Clone)]
pub struct Device1Proxy {
    address: String,
    name: String,
}

pub async fn get_device_proxy(address: &str) -> Result<Device1Proxy> {
    Ok(Device1Proxy {
        address: address.to_string(),
        name: format!("Device-{}", address),
    })
}

pub async fn connect_device(address: &str) -> Result<()> {
    tracing::info!("Connecting to device: {}", address);
    Ok(())
}

pub async fn disconnect_device(address: &str) -> Result<()> {
    tracing::info!("Disconnecting from device: {}", address);
    Ok(())
}

pub async fn pair_device(address: &str) -> Result<()> {
    tracing::info!("Pairing with device: {}", address);
    Ok(())
}

pub async fn set_trusted(address: &str, trusted: bool) -> Result<()> {
    tracing::info!("Set trusted={} for device: {}", trusted, address);
    Ok(())
}

pub async fn get_device_info(
    address: &str,
) -> Result<(String, bool, bool, bool)> {
    Ok((
        format!("Device-{}", address),
        false,
        false,
        false,
    ))
}
