use bevy::prelude::*;
use bevy_ecs_typewriter::{Typewriter, TypewriterPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(TypewriterPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, display_system)
        .run();
}

#[derive(Component)]
struct TypewriterId(usize);

fn setup(mut commands: Commands) {
    let texts = vec![
        ("快速打字机", "这是一个快速的打字机效果！", 0.03),
        ("中速打字机", "这是一个中等速度的打字机效果。", 0.08),
        (
            "慢速打字机",
            "这...是...一...个...很...慢...的...打...字...机...",
            0.2,
        ),
    ];

    for (id, (name, text, speed)) in texts.into_iter().enumerate() {
        let mut typewriter = Typewriter::new(text, speed);
        typewriter.play();

        commands.spawn((typewriter, TypewriterId(id), Name::new(name.to_string())));
    }

    info!("启动了 3 个不同速度的打字机");
}

fn display_system(query: Query<(&Typewriter, &TypewriterId, &Name), Changed<Typewriter>>) {
    for (typewriter, id, name) in &query {
        println!(
            "[打字机 {}] {} | 进度: {:.0}%\n  → {}",
            id.0,
            name,
            typewriter.progress() * 100.0,
            typewriter.current_text.replace('\n', " ")
        );
    }
}
