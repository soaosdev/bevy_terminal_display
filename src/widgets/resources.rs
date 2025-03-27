use bevy::prelude::*;

#[derive(Resource, Default, Deref, DerefMut, Debug)]
pub struct FocusedWidget(pub Option<Entity>);
