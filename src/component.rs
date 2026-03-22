//! # component.rs
//!
//! # component.rs 文件
//!
//! ## Module Overview
//!
//! ## 模块概述
//!
//! This file defines the stateful ECS data used by the typewriter effect. It contains the
//! `Typewriter` component itself together with its playback state enum and the small helper methods
//! that external code uses to start, pause, stop, or inspect the effect.
//!
//! 这个文件定义了打字机效果需要的有状态 ECS 数据。它包含 `Typewriter` 组件本体、对应的播放
//! 状态枚举，以及外部代码用来启动、暂停、停止和查询该效果的一组小型辅助方法。

use bevy::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Debug, PartialEq)]
pub enum TypewriterState {
    #[default]
    Idle,
    Playing,
    Paused,
    Finished,
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Typewriter {
    pub source_text: String,
    pub current_text: String,
    pub timer: Timer,
    pub state: TypewriterState,
    pub current_char_index: usize,
}

impl Typewriter {
    pub fn new(text: impl Into<String>, char_duration: f32) -> Self {
        Self {
            source_text: text.into(),
            current_text: String::new(),
            timer: Timer::from_seconds(char_duration, TimerMode::Repeating),
            state: TypewriterState::Idle,
            current_char_index: 0,
        }
    }

    pub fn play(&mut self) {
        if self.state == TypewriterState::Idle {
            self.current_char_index = 0;
            self.current_text.clear();
        }
        self.state = TypewriterState::Playing;
        self.timer.reset();
    }

    pub fn pause(&mut self) {
        if self.state == TypewriterState::Playing {
            self.state = TypewriterState::Paused;
        }
    }

    pub fn resume(&mut self) {
        if self.state == TypewriterState::Paused {
            self.state = TypewriterState::Playing;
        }
    }

    pub fn stop(&mut self) {
        self.state = TypewriterState::Idle;
        self.current_char_index = 0;
        self.current_text.clear();
        self.timer.reset();
    }

    pub fn restart(&mut self) {
        self.stop();
        self.play();
    }

    pub fn is_finished(&self) -> bool {
        self.state == TypewriterState::Finished
    }

    pub fn is_playing(&self) -> bool {
        self.state == TypewriterState::Playing
    }

    pub fn progress(&self) -> f32 {
        let total = self.source_text.chars().count();
        if total == 0 {
            return 1.0;
        }
        self.current_char_index as f32 / total as f32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_typewriter_is_idle() {
        let tw = Typewriter::new("hello", 0.05);
        assert_eq!(tw.state, TypewriterState::Idle);
        assert!(tw.current_text.is_empty());
        assert_eq!(tw.current_char_index, 0);
    }

    #[test]
    fn play_sets_playing_state() {
        let mut tw = Typewriter::new("hello", 0.05);
        tw.play();
        assert_eq!(tw.state, TypewriterState::Playing);
    }

    #[test]
    fn pause_and_resume() {
        let mut tw = Typewriter::new("hello", 0.05);
        tw.play();
        tw.pause();
        assert_eq!(tw.state, TypewriterState::Paused);
        tw.resume();
        assert_eq!(tw.state, TypewriterState::Playing);
    }

    #[test]
    fn stop_resets_state() {
        let mut tw = Typewriter::new("hello", 0.05);
        tw.play();
        tw.current_char_index = 3;
        tw.current_text = "hel".to_string();
        tw.stop();
        assert_eq!(tw.state, TypewriterState::Idle);
        assert_eq!(tw.current_char_index, 0);
        assert!(tw.current_text.is_empty());
    }

    #[test]
    fn restart_resets_and_plays() {
        let mut tw = Typewriter::new("hello", 0.05);
        tw.play();
        tw.current_char_index = 3;
        tw.restart();
        assert_eq!(tw.state, TypewriterState::Playing);
        assert_eq!(tw.current_char_index, 0);
    }

    #[test]
    fn progress_empty_text() {
        let tw = Typewriter::new("", 0.05);
        assert_eq!(tw.progress(), 1.0);
    }

    #[test]
    fn progress_partial() {
        let mut tw = Typewriter::new("hello", 0.05);
        tw.current_char_index = 2;
        assert!((tw.progress() - 0.4).abs() < f32::EPSILON);
    }

    #[test]
    fn pause_only_from_playing() {
        let mut tw = Typewriter::new("hello", 0.05);
        tw.pause();
        assert_eq!(tw.state, TypewriterState::Idle);
    }

    #[test]
    fn resume_only_from_paused() {
        let mut tw = Typewriter::new("hello", 0.05);
        tw.resume();
        assert_eq!(tw.state, TypewriterState::Idle);
    }
}
