use bevy::prelude::*;
use downcast_rs::{impl_downcast, DowncastSync};
use ratatui::{layout::Rect, Frame};

use crate::input::events::TerminalInputEvent;

/// Components for this module
pub mod components;

/// Resources for this module
pub mod resources;

/// Systems for this module
pub(crate) mod systems;

/// Commands for this module
pub mod commands;

/// Trait which defines an interface for terminal widgets
pub trait TerminalWidget: DowncastSync {
    /// Called every frame to render the widget
    fn render(&mut self, frame: &mut Frame, rect: Rect);

    /// Called when a terminal input event is invoked to update any state accordingly
    fn handle_events(&mut self, _event: &TerminalInputEvent, _commands: &mut Commands) {}

    /// Called every frame during the Update schedule
    fn update(&mut self, _time: &Time, _commands: &mut Commands) {}
}
impl_downcast!(sync TerminalWidget);
