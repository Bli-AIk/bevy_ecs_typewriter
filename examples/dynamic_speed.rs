use bevy::prelude::*;
use bevy_ecs_typewriter::{Typewriter, TypewriterPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(TypewriterPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, (speed_control_system, sync_text_system, status_display_system))
        .run();
}

#[derive(Component)]
struct DynamicSpeed;

#[derive(Component)]
struct StatusDisplay;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    let mut typewriter = Typewriter::new(
        "This is a typewriter with dynamic speed control.\n\nPress UP arrow to speed up.\nPress DOWN arrow to slow down.\n\nCurrent speed is displayed at the bottom.",
        0.1,
    );
    typewriter.play();
    
    commands.spawn((
        Text::new(""),
        TextFont {
            font: asset_server.load("Unifont.otf"),
            font_size: 26.0,
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
        DynamicSpeed,
    ));

    commands.spawn((
        Text::new(""),
        TextFont {
            font: asset_server.load("Unifont.otf"),
            font_size: 20.0,
            ..default()
        },
        TextColor(Color::srgb(0.7, 0.7, 0.7)),
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(50.0),
            left: Val::Px(50.0),
            ..default()
        },
        StatusDisplay,
    ));

    info!("Dynamic Speed Example Started");
    info!("Controls: SPACE - Play/Pause | R - Restart | S - Stop | UP/DOWN - Adjust Speed");
}

fn speed_control_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Typewriter, With<DynamicSpeed>>,
) {
    if let Ok(mut typewriter) = query.single_mut() {
        let current_duration = typewriter.timer.duration().as_secs_f32();
        let mut new_duration = current_duration;

        if keyboard.just_pressed(KeyCode::ArrowUp) {
            new_duration = (current_duration - 0.01).max(0.01);
            info!("Speed up! New speed: {:.3}s/char", new_duration);
        }

        if keyboard.just_pressed(KeyCode::ArrowDown) {
            new_duration = (current_duration + 0.01).min(1.0);
            info!("Slow down! New speed: {:.3}s/char", new_duration);
        }

        if new_duration != current_duration {
            typewriter.timer.set_duration(std::time::Duration::from_secs_f32(new_duration));
        }

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
    mut query: Query<(&Typewriter, &mut Text), (Changed<Typewriter>, With<DynamicSpeed>)>,
) {
    for (typewriter, mut text) in &mut query {
        **text = typewriter.current_text.clone();
    }
}

fn status_display_system(
    typewriter_query: Query<&Typewriter, With<DynamicSpeed>>,
    mut status_query: Query<&mut Text, With<StatusDisplay>>,
) {
    if let (Ok(typewriter), Ok(mut status_text)) = (typewriter_query.single(), status_query.single_mut()) {
        let speed = typewriter.timer.duration().as_secs_f32();
        **status_text = format!(
            "Speed: {:.3}s/char | Progress: {:.0}% | State: {:?}",
            speed,
            typewriter.progress() * 100.0,
            typewriter.state
        );
    }
}
