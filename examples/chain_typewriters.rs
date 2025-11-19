use bevy::prelude::*;
use bevy_ecs_typewriter::{Typewriter, TypewriterPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(TypewriterPlugin)
        .insert_resource(DialogueState::default())
        .add_systems(Startup, setup)
        .add_systems(Update, (chain_system, display_system))
        .run();
}

#[derive(Resource, Default)]
struct DialogueState {
    current_index: usize,
    dialogues: Vec<String>,
}

#[derive(Component)]
struct ChainedDialogue;

fn setup(mut commands: Commands, mut state: ResMut<DialogueState>) {
    state.dialogues = vec![
        "游戏开始...".to_string(),
        "你醒来时发现自己在一个陌生的房间里。".to_string(),
        "房间里有一扇门和一个窗户。".to_string(),
        "你听到门外传来脚步声...".to_string(),
        "突然，门被推开了！".to_string(),
        "一个神秘人影出现在门口。".to_string(),
        "「欢迎来到这个世界。」".to_string(),
        "对话结束。".to_string(),
    ];

    let mut typewriter = Typewriter::new(&state.dialogues[0], 0.05);
    typewriter.play();

    commands.spawn((typewriter, ChainedDialogue));

    info!("连续对话示例启动");
    info!("将依次播放 {} 段对话", state.dialogues.len());
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
                "播放第 {} / {} 段对话",
                state.current_index + 1,
                state.dialogues.len()
            );
        }
    }
}

fn display_system(query: Query<&Typewriter, (Changed<Typewriter>, With<ChainedDialogue>)>) {
    for typewriter in &query {
        println!("\n╔══════════════════════════════════════╗");
        println!("║ {:<36} ║", typewriter.current_text);
        println!("╚══════════════════════════════════════╝");
    }
}
