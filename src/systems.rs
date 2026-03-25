//! # systems.rs
//!
//! # systems.rs 文件
//!
//! ## Module Overview
//!
//! ## 模块概述
//!
//! Contains the runtime system that advances all active typewriter components. It is the
//! part of the crate that turns elapsed time into newly revealed characters while respecting pause,
//! finish, and restart semantics defined by the component state.
//!
//! 包含推进所有活跃 typewriter 组件的运行时系统。它负责把流逝时间转换成新揭示出的
//! 字符，同时遵守组件状态里定义的暂停、结束和重启语义。

use bevy::prelude::*;

use crate::component::{Typewriter, TypewriterState};

pub(crate) fn typewriter_system(time: Res<Time>, mut query: Query<&mut Typewriter>) {
    for mut typewriter in &mut query {
        if typewriter.state != TypewriterState::Playing {
            continue;
        }

        typewriter.timer.tick(time.delta());

        if typewriter.timer.is_finished() {
            let total_chars = typewriter.source_text.chars().count();
            if typewriter.current_char_index >= total_chars {
                typewriter.state = TypewriterState::Finished;
                continue;
            }

            let char_indices: Vec<_> = typewriter.source_text.char_indices().collect();
            let source_len = typewriter.source_text.len();

            if let Some(&(byte_index, _)) = char_indices.get(typewriter.current_char_index) {
                let next_byte_index = char_indices
                    .get(typewriter.current_char_index + 1)
                    .map(|&(i, _)| i)
                    .unwrap_or(source_len);

                let char_str = typewriter.source_text[byte_index..next_byte_index].to_string();
                typewriter.current_text.push_str(&char_str);
                typewriter.current_char_index += 1;
            }
        }
    }
}
