use anyhow::Result;
use ratatui::style::Color;
use serde::Deserialize;
use std::path::PathBuf;
use tokio::sync::mpsc;

#[derive(Debug, Clone, Deserialize)]
pub struct ThemeColors {
    pub background: String,
    pub foreground: String,
    pub accent: String,
    pub cursor: Option<String>,
    pub selection_background: Option<String>,
    pub selection_foreground: Option<String>,
    pub color0: Option<String>,
    pub color1: Option<String>,
    pub color2: Option<String>,
    pub color3: Option<String>,
    pub color4: Option<String>,
    pub color5: Option<String>,
    pub color6: Option<String>,
    pub color7: Option<String>,
    pub color8: Option<String>,
    pub color9: Option<String>,
    pub color10: Option<String>,
    pub color11: Option<String>,
    pub color12: Option<String>,
    pub color13: Option<String>,
    pub color14: Option<String>,
    pub color15: Option<String>,
}

impl ThemeColors {
    pub fn load() -> Result<Self> {
        let path = get_colors_path();
        if !path.exists() {
            return Ok(Self::fallback());
        }

        let content = std::fs::read_to_string(&path)?;
        let colors: ThemeColors = toml::from_str(&content)?;
        Ok(colors)
    }

    pub fn fallback() -> Self {
        Self {
            background: "#1e1e2e".to_string(),
            foreground: "#cdd6f4".to_string(),
            accent: "#89b4fa".to_string(),
            cursor: Some("#f5e0dc".to_string()),
            selection_background: Some("#f5e0dc".to_string()),
            selection_foreground: Some("#1e1e2e".to_string()),
            color0: Some("#45475a".to_string()),
            color1: Some("#f38ba8".to_string()),
            color2: Some("#a6e3a1".to_string()),
            color3: Some("#f9e2af".to_string()),
            color4: Some("#89b4fa".to_string()),
            color5: Some("#f5c2e7".to_string()),
            color6: Some("#94e2d5".to_string()),
            color7: Some("#bac2de".to_string()),
            color8: Some("#585b70".to_string()),
            color9: Some("#f5c2e7".to_string()),
            color10: Some("#89dceb".to_string()),
            color11: Some("#f9e2af".to_string()),
            color12: Some("#89b4fa".to_string()),
            color13: Some("#cba6f7".to_string()),
            color14: Some("#89dceb".to_string()),
            color15: Some("#f5f5f5".to_string()),
        }
    }

    pub fn bg_color(&self) -> Color {
        hex_to_color(&self.background)
    }

    pub fn fg_color(&self) -> Color {
        hex_to_color(&self.foreground)
    }

    pub fn accent_color(&self) -> Color {
        hex_to_color(&self.accent)
    }

    pub fn success_color(&self) -> Color {
        self.color2
            .as_ref()
            .map(|c| hex_to_color(c))
            .unwrap_or(Color::Green)
    }

    pub fn error_color(&self) -> Color {
        self.color1
            .as_ref()
            .map(|c| hex_to_color(c))
            .unwrap_or(Color::Red)
    }

    pub fn warning_color(&self) -> Color {
        self.color3
            .as_ref()
            .map(|c| hex_to_color(c))
            .unwrap_or(Color::Yellow)
    }

    pub fn info_color(&self) -> Color {
        self.color4
            .as_ref()
            .map(|c| hex_to_color(c))
            .unwrap_or(Color::Cyan)
    }
}

pub fn hex_to_color(hex: &str) -> Color {
    let hex = hex.trim_start_matches('#');
    if let Ok(n) = u32::from_str_radix(hex, 16) {
        let r = ((n >> 16) & 0xFF) as u8;
        let g = ((n >> 8) & 0xFF) as u8;
        let b = (n & 0xFF) as u8;
        Color::Rgb(r, g, b)
    } else {
        Color::Reset
    }
}

fn get_colors_path() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from(".config"))
        .join("omablue/current/theme/colors.toml")
}

pub async fn watch_theme(tx: mpsc::Sender<crate::events::AppEvent>) {
    use notify::{Watcher, RecursiveMode, Result as NotifyResult};
    use notify::recommended_watcher;

    let colors_path = get_colors_path();
    let colors_dir = colors_path.parent().unwrap_or_else(|| std::path::Path::new("."));

    let (watcher_tx, watcher_rx) = std::sync::mpsc::channel();

    let mut watcher = match recommended_watcher(move |_: NotifyResult<notify::Event>| {
        let _ = watcher_tx.send(());
    }) {
        Ok(w) => w,
        Err(e) => {
            tracing::warn!("Failed to create file watcher: {}", e);
            return;
        }
    };

    if let Err(e) = watcher.watch(colors_dir, RecursiveMode::NonRecursive) {
        tracing::warn!("Failed to watch theme directory: {}", e);
        return;
    }

    // Wait for file changes in a blocking thread
    std::thread::spawn(move || {
        while let Ok(()) = watcher_rx.recv() {
            std::thread::sleep(std::time::Duration::from_millis(100));
            if let Ok(colors) = ThemeColors::load() {
                let _ = tx.blocking_send(crate::events::AppEvent::ThemeChanged(colors));
            }
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_to_color() {
        assert_eq!(hex_to_color("#FF0000"), Color::Rgb(255, 0, 0));
        assert_eq!(hex_to_color("#00FF00"), Color::Rgb(0, 255, 0));
        assert_eq!(hex_to_color("0000FF"), Color::Rgb(0, 0, 255));
    }

    #[test]
    fn test_fallback_theme() {
        let theme = ThemeColors::fallback();
        assert_eq!(theme.background, "#1e1e2e");
        assert_eq!(theme.foreground, "#cdd6f4");
    }
}
