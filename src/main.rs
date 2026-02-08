use std::thread;

use anyhow::Result;
use tao::event_loop::{ControlFlow, EventLoopBuilder};
#[cfg(target_os = "macos")]
use tao::platform::macos::{ActivationPolicy, EventLoopExtMacOS};
use tokio::sync::mpsc;
use tray_icon::TrayIconBuilder;
use tray_icon::menu::{Menu, MenuItem, PredefinedMenuItem};

mod config;
mod core;
mod gmail;
mod icon;
mod logging;
mod notifier;

#[warn(clippy::collapsible_if)]
fn main() -> Result<()> {
    logging::init();

    let git_hash = option_env!("GIT_HASH").unwrap_or("DEV");
    tracing::info!("hedwig({}) is starting UI...", git_hash);

    let mut el = EventLoopBuilder::new().build();
    el.set_activation_policy(ActivationPolicy::Accessory);

    let tray_menu = Menu::new();
    let quit_i = MenuItem::new("Quit Hedwig", true, None);
    tray_menu.append_items(&[&PredefinedMenuItem::separator(), &quit_i])?;

    let mut _tray_icon = Some(
        TrayIconBuilder::new()
            .with_menu(Box::new(tray_menu))
            .with_tooltip("Hedwig - Gmail Watcher")
            .with_icon(icon::load_app_icon()?)
            .build()?,
    );
    let menu_channel = tray_icon::menu::MenuEvent::receiver();

    thread::spawn(move || {
        let rt = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("failed to build tokio runtime");
        tracing::info!("background logic started");
        if let Err(e) = rt.block_on(run_backend()) {
            tracing::error!("backend error: {}", e);
        }
    });

    el.run(move |_event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        if menu_channel.try_recv().is_ok_and(|e| e.id == quit_i.id()) {
            _tray_icon = None;
            *control_flow = ControlFlow::Exit;
        };
    });
}

async fn run_backend() -> Result<()> {
    let (tx, rx) = mpsc::channel::<notify::Event>(100);

    let _watcher = config::watcher::run(tx)?;
    core::event_loop(rx).await?;
    Ok(())
}
