use bevy::prelude::*;

use crate::component::{Typewriter, TypewriterState};
use crate::systems::typewriter_system;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypewriterSystemSet;

pub struct TypewriterPlugin;

impl Plugin for TypewriterPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Typewriter>()
            .register_type::<TypewriterState>()
            .add_systems(
                Update,
                typewriter_system
                    .in_set(TypewriterSystemSet)
                    .run_if(any_with_component::<Typewriter>),
            );
    }
}
