use bevy::prelude::*;
use bevy_ecs_typewriter::{Typewriter, TypewriterPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(TypewriterPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, sync_text_system)
        .run();
}

#[derive(Component)]
struct DialogueText;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    let mut typewriter = Typewriter::new(
        "这是一个与 UI 集成的示例。\n\
         打字机负责纯文本管理，\n\
         而 UI 组件只需同步显示文本即可。\n\
         这种解耦设计让代码更加灵活！",
        0.05,
    );
    typewriter.play();

    commands.spawn((
        Text::new(""),
        TextFont {
            font_size: 30.0,
            ..default()
        },
        TextColor(Color::WHITE),
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

    info!("UI 集成示例启动");
    info!("打字机文本将自动同步到 UI Text 组件");
}

fn sync_text_system(
    mut query: Query<(&Typewriter, &mut Text), (Changed<Typewriter>, With<DialogueText>)>,
) {
    for (typewriter, mut text) in &mut query {
        **text = typewriter.current_text.clone();
    }
}
