use anyhow::Result;
use zbus::proxy;

#[proxy(
    interface = "org.bluez.Device1",
    default_service = "org.bluez"
)]
trait Device1 {
    async fn connect(&self) -> zbus::Result<()>;
    async fn disconnect(&self) -> zbus::Result<()>;
    async fn pair(&self) -> zbus::Result<()>;

    #[zbus(property)]
    async fn name(&self) -> zbus::Result<String>;

    #[zbus(property)]
    async fn address(&self) -> zbus::Result<String>;

    #[zbus(property)]
    async fn connected(&self) -> zbus::Result<bool>;

    #[zbus(property)]
    async fn trusted(&self) -> zbus::Result<bool>;

    #[zbus(property)]
    async fn set_trusted(&self, value: bool) -> zbus::Result<()>;

    #[zbus(property)]
    async fn paired(&self) -> zbus::Result<bool>;

    #[zbus(property)]
    async fn adapter(&self) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;
}

pub async fn get_device_proxy(address: &str) -> Result<Device1Proxy<'static>> {
    let connection = zbus::Connection::system().await?;
    let path = format!("/org/bluez/hci0/dev_{}", address.replace(":", "_"));
    let proxy = Device1Proxy::builder(&connection)
        .path(path.as_str())?
        .build()
        .await?;
    Ok(proxy)
}

pub async fn connect_device(address: &str) -> Result<()> {
    let device = get_device_proxy(address).await?;
    device.connect().await?;
    tracing::info!("Connecting to device: {}", address);
    Ok(())
}

pub async fn disconnect_device(address: &str) -> Result<()> {
    let device = get_device_proxy(address).await?;
    device.disconnect().await?;
    tracing::info!("Disconnecting from device: {}", address);
    Ok(())
}

pub async fn pair_device(address: &str) -> Result<()> {
    let device = get_device_proxy(address).await?;
    device.pair().await?;
    tracing::info!("Pairing with device: {}", address);
    Ok(())
}

pub async fn set_trusted(address: &str, trusted: bool) -> Result<()> {
    let device = get_device_proxy(address).await?;
    device.set_trusted(trusted).await?;
    tracing::info!("Set trusted={} for device: {}", trusted, address);
    Ok(())
}

pub async fn get_device_info(
    address: &str,
) -> Result<(String, bool, bool, bool)> {
    let device = get_device_proxy(address).await?;
    let name = device.name().await?;
    let connected = device.connected().await?;
    let paired = device.paired().await?;
    let trusted = device.trusted().await?;
    Ok((name, connected, paired, trusted))
}
