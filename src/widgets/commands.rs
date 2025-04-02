use bevy::prelude::*;

use super::resources::FocusedWidget;

struct FocusWidgetCommand(Entity);

impl Command for FocusWidgetCommand {
    fn apply(self, world: &mut World) {
        **world.resource_mut::<FocusedWidget>() = Some(self.0);
    }
}

/// Command interface for manipulating terminal widget resources
pub trait TerminalWidgetCommands {
    /// Gives focus to the terminal widget on the provided entity.
    fn focus_widget(&mut self, widget: Entity);
}

impl TerminalWidgetCommands for Commands<'_, '_> {
    fn focus_widget(&mut self, widget: Entity) {
        self.queue(FocusWidgetCommand(widget));
    }
}
