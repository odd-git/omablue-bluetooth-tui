use crate::app::AppState;
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph, Wrap};

pub fn render_main_layout(f: &mut Frame, app: &AppState) {
    let size = f.area();

    // Main layout: 3 rows (top, middle, bottom)
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Min(10),
                Constraint::Length(1),
                Constraint::Length(2),
            ]
            .as_ref(),
        )
        .split(size);

    // Top: two columns (device list + device info)
    render_device_list_panel(f, app, chunks[0]);

    // Middle: status bar
    render_status_bar(f, app, chunks[1]);

    // Bottom: help footer
    render_help_footer(f, app, chunks[2]);
}

fn render_device_list_panel(f: &mut Frame, app: &AppState, area: Rect) {
    let columns = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    // Left panel: paired devices
    render_paired_devices(f, app, columns[0]);

    // Right panel: device details
    render_device_details(f, app, columns[1]);
}

fn render_paired_devices(f: &mut Frame, app: &AppState, area: Rect) {
    let items: Vec<ListItem> = app
        .devices
        .iter()
        .enumerate()
        .map(|(idx, device)| {
            let icon = if device.connected { "󰂱 " } else { "󰂲 " };
            let battery = device
                .battery
                .map(|b| format!(" 󰁹{}%", b))
                .unwrap_or_default();
            let content = format!("{}{}{}", icon, device.name, battery);

            let style = if idx == app.selected {
                Style::default()
                    .fg(app.theme.bg_color())
                    .bg(app.theme.accent_color())
            } else if device.connected {
                Style::default().fg(app.theme.success_color())
            } else {
                Style::default().fg(app.theme.fg_color())
            };

            ListItem::new(content).style(style)
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .title("Paired Devices")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(app.theme.accent_color())),
        )
        .style(Style::default().fg(app.theme.fg_color()));

    f.render_widget(list, area);
}

fn render_device_details(f: &mut Frame, app: &AppState, area: Rect) {
    let content = if let Some(device) = app.devices.get(app.selected) {
        format!(
            "Name:     {}\nMAC:      {}\nState:    {}\nPaired:   {}\nTrusted:  {}\nBattery:  {}",
            device.name,
            device.address,
            if device.connected { "Connected" } else { "Disconnected" },
            if device.paired { "Yes" } else { "No" },
            if device.trusted { "Yes" } else { "No" },
            device
                .battery
                .map(|b| format!("{}%", b))
                .unwrap_or_else(|| "N/A".to_string())
        )
    } else {
        "No devices selected".to_string()
    };

    let paragraph = Paragraph::new(content)
        .block(
            Block::default()
                .title("Device Info")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(app.theme.accent_color())),
        )
        .style(Style::default().fg(app.theme.fg_color()))
        .wrap(Wrap { trim: true });

    f.render_widget(paragraph, area);
}

fn render_status_bar(f: &mut Frame, app: &AppState, area: Rect) {
    let adapter_status = if app.adapter_powered {
        "Adapter: ON"
    } else {
        "Adapter: OFF"
    };

    let scan_status = if app.scanning {
        " | Scan: ACTIVE"
    } else {
        " | Scan: Inactive"
    };

    let spinner = app.get_spinner();
    let operation_status = if app.operation_in_progress {
        format!(" {} {}", spinner, app.last_action.as_ref().unwrap_or(&"Processing".to_string()))
    } else {
        String::new()
    };

    let bar = if let Some(msg) = &app.status_msg {
        format!("{}{}{} | {}", adapter_status, scan_status, operation_status, msg)
    } else {
        format!("{}{}{}", adapter_status, scan_status, operation_status)
    };

    let color = if app.error_msg.is_some() {
        app.theme.error_color()
    } else {
        app.theme.fg_color()
    };

    let paragraph = Paragraph::new(bar).style(
        Style::default()
            .fg(color)
            .bg(app.theme.bg_color()),
    );

    f.render_widget(paragraph, area);
}

fn render_help_footer(f: &mut Frame, app: &AppState, area: Rect) {
    let help_text = "[↑/↓] Nav  [⏎] Connect  [d] Disconnect  [s] Scan  [t] Toggle BT  [q] Quit";

    let paragraph = Paragraph::new(help_text)
        .style(
            Style::default()
                .fg(app.theme.accent_color())
                .bg(app.theme.bg_color()),
        )
        .alignment(Alignment::Center);

    f.render_widget(paragraph, area);
}
