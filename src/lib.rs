#![warn(missing_docs)]

//! Bevy plugin which allows a camera to render to a terminal window.

use std::{
    fs::OpenOptions, io::{stdout, Write}, path::PathBuf,
};

use bevy::{
    log::{
        tracing_subscriber::{self, layer::SubscriberExt, EnvFilter, Layer, Registry},
        Level,
    }, prelude::*, utils::tracing::subscriber,
};
use bevy_dither_post_process::DitherPostProcessPlugin;

use bevy_headless_render::HeadlessRenderPlugin;
use color_eyre::config::HookBuilder;
pub use crossterm;
use crossterm::{event::{DisableMouseCapture, PopKeyboardEnhancementFlags}, terminal::{disable_raw_mode, LeaveAlternateScreen}, ExecutableCommand};
pub use ratatui;

/// Functions and types related to capture and display of world to terminal
pub mod display;

/// Functions and types related to capturing and processing user keyboard input
pub mod input;

/// Functions and types related to constructing and rendering TUI widgets
pub mod widgets;

/// Plugin providing terminal display functionality
pub struct TerminalDisplayPlugin {
    /// Path to redirect tracing logs to. Defaults to "debug.log"
    pub log_path: PathBuf,
}

impl Default for TerminalDisplayPlugin {
    fn default() -> Self {
        Self {
            log_path: "debug.log".into(),
        }
    }
}

impl Plugin for TerminalDisplayPlugin {
    fn build(&self, app: &mut App) {
        let log_path = self.log_path.clone();
        let log_file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(log_path)
            .unwrap();
        let file_layer = tracing_subscriber::fmt::Layer::new()
            .with_writer(log_file)
            .with_filter(EnvFilter::builder().parse_lossy(format!("{},{}", Level::INFO, "wgpu=error,naga=warn")));
        let subscriber = Registry::default().with(file_layer);
        subscriber::set_global_default(subscriber).unwrap();

        let (panic, error) = HookBuilder::default().into_hooks();
        let panic = panic.into_panic_hook();
        let error = error.into_eyre_hook();

        color_eyre::eyre::set_hook(Box::new(move |e| {
            let _ = restore_terminal();
            error(e)
        })).unwrap();

        std::panic::set_hook(Box::new(move |info| {
            let _ = restore_terminal();
            error!("{info}");
            panic(info);
        }));
        
        app.add_plugins((
            DitherPostProcessPlugin,
            HeadlessRenderPlugin,
        ))
        .add_systems(Startup, input::systems::setup_input)
        .add_systems(
            Update,
            (
                input::systems::input_handling,
                display::systems::resize_handling,
                display::systems::print_to_terminal,
                widgets::systems::widget_input_handling,
                widgets::systems::update_widgets,
            ),
        )
        .insert_resource(display::resources::Terminal::default())
        .insert_resource(input::resources::EventQueue::default())
        .init_resource::<widgets::resources::FocusedWidget>()
        .add_event::<input::events::TerminalInputEvent>();
    }
}

fn restore_terminal() {
    let _ = disable_raw_mode();
    let mut stdout = stdout();
    let _ = stdout.execute(PopKeyboardEnhancementFlags).unwrap()
        .execute(DisableMouseCapture).unwrap()
        .execute(LeaveAlternateScreen).unwrap()
        .flush();
}
