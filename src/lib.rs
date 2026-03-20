mod component;
mod plugin;
mod systems;

pub mod prelude {
    pub use crate::{Typewriter, TypewriterPlugin, TypewriterState, TypewriterSystemSet};
}

pub use component::{Typewriter, TypewriterState};
pub use plugin::{TypewriterPlugin, TypewriterSystemSet};
