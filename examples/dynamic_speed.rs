use bevy::prelude::*;
use bevy_ecs_typewriter::{Typewriter, TypewriterPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(TypewriterPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, (speed_control_system, display_system))
        .run();
}

#[derive(Component)]
struct DynamicSpeed;

fn setup(mut commands: Commands) {
    let mut typewriter = Typewriter::new(
        "这是一个可以动态调整速度的打字机。\n按上箭头加速，按下箭头减速。\n当前速度会实时显示。",
        0.1,
    );
    typewriter.play();

    commands.spawn((typewriter, DynamicSpeed));

    info!("动态速度示例启动");
    info!("使用上/下箭头键调整打字速度");
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
            info!("⚡ 加速！新速度: {:.3}秒/字符", new_duration);
        }

        if keyboard.just_pressed(KeyCode::ArrowDown) {
            new_duration = (current_duration + 0.01).min(1.0);
            info!("🐢 减速！新速度: {:.3}秒/字符", new_duration);
        }

        if new_duration != current_duration {
            typewriter
                .timer
                .set_duration(std::time::Duration::from_secs_f32(new_duration));
        }

        if keyboard.just_pressed(KeyCode::Space) {
            if typewriter.is_playing() {
                typewriter.pause();
                info!("⏸️ 暂停");
            } else {
                typewriter.resume();
                info!("▶️ 继续");
            }
        }
    }
}

fn display_system(query: Query<&Typewriter, (Changed<Typewriter>, With<DynamicSpeed>)>) {
    for typewriter in &query {
        let speed = typewriter.timer.duration().as_secs_f32();
        println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("{}", typewriter.current_text);
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!(
            "速度: {:.3}秒/字符 | 进度: {:.0}% | 状态: {:?}",
            speed,
            typewriter.progress() * 100.0,
            typewriter.state
        );
    }
}
