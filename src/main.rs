mod app;
mod events;
mod bluetooth;
mod ui;
mod notifications;

use anyhow::Result;
use crossterm::{
    event::EnableMouseCapture,
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::prelude::*;
use std::io::Stdout;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing_subscriber::filter::LevelFilter::WARN.into()),
        )
        .init();

    let mut terminal = setup_terminal()?;
    let result = run_app(&mut terminal).await;
    restore_terminal(&mut terminal)?;

    result
}

async fn run_app(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
    let (tx, mut rx) = mpsc::channel(128);

    // Load initial app state and theme
    let mut app = app::AppState::new().await?;

    // Spawn background tasks
    let bluetooth_tx = tx.clone();
    tokio::spawn(bluetooth::run(bluetooth_tx));

    let theme_tx = tx.clone();
    tokio::spawn(ui::theme::watch_theme(theme_tx));

    let input_tx = tx.clone();
    tokio::spawn(events::read_keys(input_tx));

    loop {
        terminal.draw(|f| ui::render(f, &app))?;

        match rx.recv().await {
            Some(events::AppEvent::Key(key)) => {
                if let Some(action) = app.handle_key(key) {
                    bluetooth::dispatch(&action).await?;
                }

                if app.should_quit() {
                    break;
                }
            }
            Some(events::AppEvent::BluetoothEvent(bt_event)) => {
                app.apply_bluetooth_event(bt_event);
            }
            Some(events::AppEvent::ThemeChanged(colors)) => {
                app.set_theme(colors);
            }
            Some(events::AppEvent::Tick) => {
                app.animate_spinner();
            }
            Some(events::AppEvent::Quit) | None => break,
        }
    }

    Ok(())
}

fn setup_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>> {
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;
    Ok(terminal)
}

fn restore_terminal(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        crossterm::event::DisableMouseCapture
    )?;
    Ok(())
}
