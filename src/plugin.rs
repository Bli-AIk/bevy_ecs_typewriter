//! # plugin.rs
//!
//! # plugin.rs 文件
//!
//! ## Module Overview
//!
//! ## 模块概述
//!
//! This file defines the Bevy plugin surface for the typewriter crate. It registers the reflected
//! component types and installs the update system set that advances visible text over time.
//!
//! 这个文件定义了 typewriter crate 的 Bevy 插件入口。它会注册可反射的组件类型，并安装负责
//! 推进可见文本的更新系统集。

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
