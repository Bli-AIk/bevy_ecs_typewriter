//! This example shows several typewriter components running side by side.
//! It is meant to demonstrate that each entity keeps independent reveal timing
//! and controls, even when all of them share the same plugin and frame updates.
//!
//! 这个示例展示多个打字机组件并行运行的场景。它说明即使所有实体共享同一个插件和帧更新，
//! 每个实体依然可以保持各自独立的揭示节奏和控制状态。

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
struct TypewriterId(#[allow(dead_code)] usize);

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((Name::new("Camera"), Camera2d));

    let texts = vec![
        ("Fast Typewriter", "This is a fast typewriter effect!", 0.03),
        (
            "Medium Typewriter",
            "This is a medium-speed typewriter effect.",
            0.08,
        ),
        (
            "Slow Typewriter",
            "This is a very slow typewriter effect.",
            0.2,
        ),
    ];

    for (id, (name, text, speed)) in texts.into_iter().enumerate() {
        let mut typewriter = Typewriter::new(text, speed);
        typewriter.play();

        commands.spawn((
            Text::new(""),
            TextFont {
                font: asset_server.load("Unifont.otf"),
                font_size: 24.0,
                ..default()
            },
            TextColor(Color::srgb(0.9, 0.9, 0.9)),
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(50.0 + id as f32 * 100.0),
                left: Val::Px(50.0),
                max_width: Val::Px(700.0),
                ..default()
            },
            typewriter,
            TypewriterId(id),
            Name::new(name.to_string()),
        ));
    }

    info!("Multiple Typewriters Example Started");
    info!("Controls: SPACE - Play/Pause All | R - Restart All | S - Stop All");
}

fn control_system(keyboard: Res<ButtonInput<KeyCode>>, mut query: Query<&mut Typewriter>) {
    if keyboard.just_pressed(KeyCode::Space) {
        for mut typewriter in &mut query {
            match typewriter.state {
                bevy_ecs_typewriter::TypewriterState::Idle => {
                    info!("Playing all typewriters");
                    typewriter.play();
                }
                bevy_ecs_typewriter::TypewriterState::Playing => {
                    info!("Pausing all typewriters");
                    typewriter.pause();
                }
                bevy_ecs_typewriter::TypewriterState::Paused => {
                    info!("Resuming all typewriters");
                    typewriter.resume();
                }
                bevy_ecs_typewriter::TypewriterState::Finished => {
                    info!("Restarting all typewriters");
                    typewriter.restart();
                }
            }
        }
    }

    if keyboard.just_pressed(KeyCode::KeyR) {
        info!("Restarting all typewriters");
        for mut typewriter in &mut query {
            typewriter.restart();
        }
    }

    if keyboard.just_pressed(KeyCode::KeyS) {
        info!("Stopping all typewriters");
        for mut typewriter in &mut query {
            typewriter.stop();
        }
    }
}

fn sync_text_system(mut query: Query<(&Typewriter, &Name, &mut Text), Changed<Typewriter>>) {
    for (typewriter, name, mut text) in &mut query {
        **text = format!("[{}]\n{}", name, typewriter.current_text);
    }
}
