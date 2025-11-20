use bevy::prelude::*;
use bevy_ecs_typewriter::{Typewriter, TypewriterPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(TypewriterPlugin)
        .insert_resource(DialogueState::default())
        .add_systems(Startup, setup)
        .add_systems(Update, (control_system, chain_system, sync_text_system))
        .run();
}

#[derive(Resource, Default)]
struct DialogueState {
    current_index: usize,
    dialogues: Vec<String>,
}

#[derive(Component)]
struct ChainedDialogue;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut state: ResMut<DialogueState>) {
    commands.spawn(Camera2d);

    state.dialogues = vec![
        "Game started...".to_string(),
        "You wake up in an unfamiliar room.".to_string(),
        "There is a door and a window in the room.".to_string(),
        "You hear footsteps outside the door...".to_string(),
        "Suddenly, the door swings open!".to_string(),
        "A mysterious figure appears in the doorway.".to_string(),
        "\"Welcome to this world.\"".to_string(),
        "Dialogue ended.".to_string(),
    ];

    let mut typewriter = Typewriter::new(&state.dialogues[0], 0.05);
    typewriter.play();

    commands.spawn((
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
        ChainedDialogue,
    ));

    info!(
        "Chain Typewriters Example Started - {} dialogues",
        state.dialogues.len()
    );
    info!("Controls: SPACE - Play/Pause | R - Restart | S - Stop");
}

fn control_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Typewriter, With<ChainedDialogue>>,
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

fn chain_system(
    mut query: Query<&mut Typewriter, With<ChainedDialogue>>,
    mut state: ResMut<DialogueState>,
) {
    if let Ok(mut typewriter) = query.single_mut() {
        if typewriter.is_finished() && state.current_index < state.dialogues.len() - 1 {
            state.current_index += 1;
            typewriter.source_text = state.dialogues[state.current_index].clone();
            typewriter.restart();

            info!(
                "Playing dialogue {} / {}",
                state.current_index + 1,
                state.dialogues.len()
            );
        }
    }
}

fn sync_text_system(
    mut query: Query<(&Typewriter, &mut Text), (Changed<Typewriter>, With<ChainedDialogue>)>,
    state: Res<DialogueState>,
) {
    for (typewriter, mut text) in &mut query {
        **text = format!(
            "[Dialogue {} / {}]\n\n{}",
            state.current_index + 1,
            state.dialogues.len(),
            typewriter.current_text
        );
    }
}
