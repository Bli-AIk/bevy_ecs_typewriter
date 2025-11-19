use bevy::prelude::*;
use bevy_ecs_typewriter::{Typewriter, TypewriterPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(TypewriterPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, (sync_text_system, log_finished_system))
        .run();
}

#[derive(Component)]
struct Language(String);

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    let multilingual_texts = vec![
        ("中文", "你好世界！这是一个支持多语言的打字机效果。"),
        ("English", "Hello World! This is a multilingual typewriter effect. "),
        ("日本語", "こんにちは世界！これは多言語タイプライター効果です。"),
        ("한국어", "안녕하세요 세상! 이것은 다국어 타자기 효과입니다. "),
        ("Русский", "Привет мир! Это многоязычный эффект печатной машинки. "),
        ("العربية", "مرحبا بالعالم! هذا تأثير آلة كاتبة متعدد اللغات."),
        ("Mixed", "Hello 世界！Привет мир  こんにちは 안녕 مرحبا"),
    ];

    for (id, (lang, text)) in multilingual_texts.into_iter().enumerate() {
        let mut typewriter = Typewriter::new(text, 0.08);
        typewriter.play();
        
        commands.spawn((
            Text::new(""),
            TextFont {
                font: asset_server.load("Unifont.otf"),
                font_size: 22.0,
                ..default()
            },
            TextColor(Color::srgb(0.9, 0.9, 0.9)),
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(50.0 + id as f32 * 70.0),
                left: Val::Px(50.0),
                max_width: Val::Px(700.0),
                ..default()
            },
            typewriter,
            Language(lang.to_string()),
        ));
    }

    info!("Multilingual Example Started");
    info!("Testing Chinese, English, Japanese, Korean, Russian, Arabic, Emoji and Mixed text");
}

fn sync_text_system(mut query: Query<(&Typewriter, &Language, &mut Text), Changed<Typewriter>>) {
    for (typewriter, language, mut text) in &mut query {
        **text = format!("[{}] {}", language.0, typewriter.current_text);
    }
}

fn log_finished_system(query: Query<(&Typewriter, &Language), Changed<Typewriter>>) {
    for (typewriter, language) in &query {
        if typewriter.is_finished() {
            info!("[{}] Finished: {}", language.0, typewriter.current_text);
        }
    }
}
