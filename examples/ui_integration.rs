//! This example demonstrates the intended separation between the typewriter state
//! machine and presentation UI. The `Typewriter` component owns reveal progress,
//! while the Bevy UI text node simply mirrors the currently visible string.
//!
//! 这个示例演示打字机状态机与展示层 UI 之间的预期分工。`Typewriter` 组件负责维护揭示进度，
//! 而 Bevy 的 UI 文本节点只负责同步并显示当前可见字符串。

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
struct DialogueText;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((Name::new("Camera"), Camera2d));

    let mut typewriter = Typewriter::new(
        "This is an example of UI integration.\n\nThe Typewriter component handles pure text management,\nwhile the UI component simply syncs and displays the text.\n\nThis decoupled design makes the code more flexible!",
        0.05,
    );
    typewriter.play();

    commands.spawn((
        Name::new("DialogueText"),
        Text::new(""),
        TextFont {
            font: asset_server.load("Unifont.otf"),
            font_size: 28.0,
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
        DialogueText,
    ));

    info!("UI Integration Example Started");
    info!("Controls: SPACE - Play/Pause | R - Restart | S - Stop");
}

fn control_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Typewriter, With<DialogueText>>,
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
    mut query: Query<(&Typewriter, &mut Text), (Changed<Typewriter>, With<DialogueText>)>,
) {
    for (typewriter, mut text) in &mut query {
        **text = typewriter.current_text.clone();
    }
}
