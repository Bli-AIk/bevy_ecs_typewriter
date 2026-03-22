//! This example demonstrates the smallest interactive setup for `bevy_ecs_typewriter`.
//! It shows how to attach a `Typewriter` component, mirror its visible text into UI,
//! and control playback with a few keyboard shortcuts.
//!
//! 这个示例展示了 `bevy_ecs_typewriter` 最小可交互接入方式。它演示如何挂载
//! `Typewriter` 组件、把可见文本同步到 UI，并通过几个键盘快捷键控制播放流程。

use bevy::prelude::*;
use bevy_ecs_typewriter::{Typewriter, TypewriterPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(TypewriterPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, (control_system, sync_text_system))
        .run();
}

#[derive(Component)]
struct PlayerControlled;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((Name::new("Camera"), Camera2d));

    let mut typewriter = Typewriter::new(
        "Welcome to the Typewriter Effect!\n\nControls:\n  SPACE - Play/Pause\n  R - Restart\n  S - Stop",
        0.05,
    );
    typewriter.play();

    commands.spawn((
        Name::new("TypewriterText"),
        Text::new(""),
        TextFont {
            font: asset_server.load("Unifont.otf"),
            font_size: 30.0,
            ..default()
        },
        TextColor(Color::srgb(0.9, 0.9, 0.9)),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(50.0),
            left: Val::Px(50.0),
            max_width: Val::Px(700.0),
            ..default()
        },
        typewriter,
        PlayerControlled,
    ));

    info!("Basic Control Example Started");
    info!("Controls: SPACE - Play/Pause | R - Restart | S - Stop");
}

fn control_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Typewriter, With<PlayerControlled>>,
) {
    if let Ok(mut typewriter) = query.single_mut() {
        if keyboard.just_pressed(KeyCode::Space) {
            match typewriter.state {
                bevy_ecs_typewriter::TypewriterState::Idle => {
                    info!("Playing");
                    typewriter.play();
                }
                bevy_ecs_typewriter::TypewriterState::Playing => {
                    info!("Paused");
                    typewriter.pause();
                }
                bevy_ecs_typewriter::TypewriterState::Paused => {
                    info!("Resumed");
                    typewriter.resume();
                }
                bevy_ecs_typewriter::TypewriterState::Finished => {
                    info!("Restarted");
                    typewriter.restart();
                }
            }
        }

        if keyboard.just_pressed(KeyCode::KeyR) {
            info!("Restarted");
            typewriter.restart();
        }

        if keyboard.just_pressed(KeyCode::KeyS) {
            info!("Stopped");
            typewriter.stop();
        }
    }
}

fn sync_text_system(
    mut query: Query<(&Typewriter, &mut Text), (Changed<Typewriter>, With<PlayerControlled>)>,
) {
    for (typewriter, mut text) in &mut query {
        **text = typewriter.current_text.clone();
    }
}
