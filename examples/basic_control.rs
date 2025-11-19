use bevy::prelude::*;
use bevy_ecs_typewriter::{Typewriter, TypewriterPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(TypewriterPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, (control_system, display_system))
        .run();
}

#[derive(Component)]
struct PlayerControlled;

fn setup(mut commands: Commands) {
    commands.spawn((
        Typewriter::new(
            "欢迎使用打字机效果！\n按空格键：播放/暂停\n按 R 键：重新开始\n按 S 键：停止",
            0.05,
        ),
        PlayerControlled,
    ));

    info!("示例启动成功！使用以下按键控制打字机：");
    info!("空格键 - 播放/暂停");
    info!("R 键 - 重新开始");
    info!("S 键 - 停止");
}

fn control_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Typewriter, With<PlayerControlled>>,
) {
    if let Ok(mut typewriter) = query.single_mut() {
        if keyboard.just_pressed(KeyCode::Space) {
            match typewriter.state {
                bevy_ecs_typewriter::TypewriterState::Idle => {
                    info!("▶️ 开始播放");
                    typewriter.play();
                }
                bevy_ecs_typewriter::TypewriterState::Playing => {
                    info!("⏸️ 暂停");
                    typewriter.pause();
                }
                bevy_ecs_typewriter::TypewriterState::Paused => {
                    info!("▶️ 继续播放");
                    typewriter.resume();
                }
                bevy_ecs_typewriter::TypewriterState::Finished => {
                    info!("🔄 重新开始");
                    typewriter.restart();
                }
            }
        }

        if keyboard.just_pressed(KeyCode::KeyR) {
            info!("🔄 重新开始");
            typewriter.restart();
        }

        if keyboard.just_pressed(KeyCode::KeyS) {
            info!("⏹️ 停止");
            typewriter.stop();
        }
    }
}

fn display_system(mut query: Query<&Typewriter, Changed<Typewriter>>) {
    for typewriter in &mut query {
        println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("{}", typewriter.current_text);
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!(
            "状态: {:?} | 进度: {:.1}%",
            typewriter.state,
            typewriter.progress() * 100.0
        );
    }
}
