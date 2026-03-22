//! # lib.rs
//!
//! # lib.rs 文件
//!
//! ## Module Overview
//!
//! ## 模块概述
//!
//! This is the public entry point for `bevy_ecs_typewriter`. It wires together the component,
//! plugin, and system modules, and exposes a small prelude so downstream crates can add the
//! typewriter effect without importing internal paths.
//!
//! 这是 `bevy_ecs_typewriter` 的公开入口。它把组件、插件和系统模块组织起来，并导出一个小型
//! prelude，方便下游 crate 在不依赖内部路径的前提下接入打字机效果。

mod component;
mod plugin;
mod systems;

pub mod prelude {
    pub use crate::{Typewriter, TypewriterPlugin, TypewriterState, TypewriterSystemSet};
}

pub use component::{Typewriter, TypewriterState};
pub use plugin::{TypewriterPlugin, TypewriterSystemSet};
