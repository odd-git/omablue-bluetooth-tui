use anyhow::Result;
use notify_rust::{Hint, Notification};

pub fn notify_success(message: &str) -> Result<()> {
    Notification::new()
        .appname("Bluetooth")
        .summary(message)
        .icon("bluetooth-symbolic")
        .hint(Hint::Custom(
            "x-dunst-stack-tag".into(),
            "bt".into(),
        ))
        .show()?;
    Ok(())
}

pub fn notify_error(message: &str) -> Result<()> {
    Notification::new()
        .appname("Bluetooth")
        .summary("Error")
        .body(message)
        .icon("dialog-error-symbolic")
        .hint(Hint::Custom(
            "x-dunst-stack-tag".into(),
            "bt".into(),
        ))
        .urgency(notify_rust::Urgency::Critical)
        .show()?;
    Ok(())
}

pub fn notify_info(message: &str) -> Result<()> {
    Notification::new()
        .appname("Bluetooth")
        .summary(message)
        .icon("bluetooth-symbolic")
        .hint(Hint::Custom(
            "x-dunst-stack-tag".into(),
            "bt".into(),
        ))
        .show()?;
    Ok(())
}
