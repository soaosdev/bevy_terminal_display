use bevy::prelude::*;

/// Terminal widget entity currently focused and handling input
/// Can be manipulated directly or you can request an entity be focused through
/// the `focus_widget` command.
#[derive(Resource, Default, Deref, DerefMut, Debug)]
pub struct FocusedWidget(pub Option<Entity>);
