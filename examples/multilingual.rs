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
struct Language(String);

fn setup(mut commands: Commands) {
    let multilingual_texts = vec![
        ("中文", "你好世界！这是一个支持多语言的打字机效果。🌏"),
        (
            "English",
            "Hello World! This is a multilingual typewriter effect. 🌍",
        ),
        (
            "日本語",
            "こんにちは世界！これは多言語タイプライター効果です。🌎",
        ),
        (
            "한국어",
            "안녕하세요 세상! 이것은 다국어 타자기 효과입니다. 🌏",
        ),
        (
            "Русский",
            "Привет мир! Это многоязычный эффект печатной машинки. 🌍",
        ),
        (
            "العربية",
            "مرحبا بالعالم! هذا تأثير آلة كاتبة متعدد اللغات. 🌎",
        ),
        ("Emoji", "🎮🎨🎭🎪🎯🎲🎰🎳🚀🚁🚂🚃🚄🚅🚆🚇🚈🚉"),
        ("Mixed", "Hello 世界！Привет мир 🌏 こんにちは 안녕 مرحبا"),
    ];

    for (lang, text) in multilingual_texts {
        let mut typewriter = Typewriter::new(text, 0.08);
        typewriter.play();

        commands.spawn((typewriter, Language(lang.to_string())));
    }

    info!("启动了多语言打字机示例");
    info!("测试了中文、英文、日文、韩文、俄文、阿拉伯文、Emoji 和混合文本");
}

fn display_system(query: Query<(&Typewriter, &Language), Changed<Typewriter>>) {
    for (typewriter, language) in &query {
        if typewriter.is_finished() {
            println!(
                "\n✅ [{}] 完成!\n   {}",
                language.0, typewriter.current_text
            );
        }
    }
}
