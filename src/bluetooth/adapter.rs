use anyhow::Result;
use zbus::proxy;

#[proxy(
    interface = "org.bluez.Adapter1",
    default_service = "org.bluez",
    default_path = "/org/bluez/hci0"
)]
trait Adapter1 {
    async fn start_discovery(&self) -> zbus::Result<()>;
    async fn stop_discovery(&self) -> zbus::Result<()>;

    #[zbus(property)]
    async fn powered(&self) -> zbus::Result<bool>;

    #[zbus(property)]
    async fn set_powered(&self, value: bool) -> zbus::Result<()>;

    #[zbus(property)]
    async fn discovering(&self) -> zbus::Result<bool>;
}

pub async fn get_adapter() -> Result<Adapter1Proxy<'static>> {
    let connection = zbus::Connection::system().await?;
    let proxy = Adapter1Proxy::new(&connection).await?;
    Ok(proxy)
}

pub async fn start_discovery() -> Result<()> {
    let adapter = get_adapter().await?;
    adapter.start_discovery().await?;
    tracing::info!("Discovery started");
    Ok(())
}

pub async fn stop_discovery() -> Result<()> {
    let adapter = get_adapter().await?;
    adapter.stop_discovery().await?;
    tracing::info!("Discovery stopped");
    Ok(())
}

pub async fn toggle_power() -> Result<()> {
    let adapter = get_adapter().await?;
    let powered = adapter.powered().await?;
    adapter.set_powered(!powered).await?;
    tracing::info!("Adapter power: {}", !powered);
    Ok(())
}

pub async fn is_powered() -> Result<bool> {
    let adapter = get_adapter().await?;
    Ok(adapter.powered().await?)
}

pub async fn is_discovering() -> Result<bool> {
    let adapter = get_adapter().await?;
    Ok(adapter.discovering().await?)
}
