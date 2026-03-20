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
