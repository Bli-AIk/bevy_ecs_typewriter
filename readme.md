# bevy_ecs_typewriter

[![license](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue)](LICENSE-APACHE) <img src="https://img.shields.io/github/repo-size/Bli-AIk/souprune.svg"/> <img src="https://img.shields.io/github/last-commit/Bli-AIk/souprune.svg"/> <br>
<img src="https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white" />

**bevy_ecs_typewriter** — A pure text typewriter effect plugin for Bevy.

| English | Simplified Chinese          |
|---------|-----------------------------|
| English | [简体中文](./readme_zh-hans.md) |

## Introduction

`bevy_ecs_typewriter` is a lightweight typewriter effect plugin for the Bevy game engine.  
It provides pure text management without any dependency on UI components, making it flexible for various use cases.

With `bevy_ecs_typewriter`, you can create typewriter effects for dialogues, subtitles, or any text animations with simple component-based control.

## Features

* 🎮 **Pure Text Management** - No dependency on UI components, works with any rendering solution
* ⚡ **Simple API** - Easy to use with play, pause, resume, and stop controls
* 🌏 **Full Unicode Support** - Works with any language including CJK, Arabic, Emoji, etc.
* 🔄 **Multiple Typewriters** - Run multiple typewriters simultaneously
* ⏱️ **Dynamic Speed Control** - Adjust typing speed in real-time
* 📊 **Progress Tracking** - Get current progress and state information

## How to Use

1. **Add to Cargo.toml**:

   ```toml
   [dependencies]
   bevy = "0.17.2"
   bevy_ecs_typewriter = "0.0.0"
   ```

2. **Add the plugin to your app**:

   ```rust
   use bevy::prelude::*;
   use bevy_ecs_typewriter::{Typewriter, TypewriterPlugin};

   fn main() {
       App::new()
           .add_plugins(DefaultPlugins)
           .add_plugins(TypewriterPlugin)
           .add_systems(Startup, setup)
           .run();
   }
   ```

3. **Create a typewriter entity**:

   ```rust
   fn setup(mut commands: Commands) {
       let mut typewriter = Typewriter::new("Hello, World!", 0.1);
       typewriter.play();
       commands.spawn(typewriter);
   }
   ```

4. **Access the current text**:

   ```rust
   fn display_system(query: Query<&Typewriter, Changed<Typewriter>>) {
       for typewriter in &query {
           println!("{}", typewriter.current_text);
       }
   }
   ```

## Examples

Run the examples to see the plugin in action:

```bash
# Basic control with keyboard input
cargo run --example basic_control

# Multiple typewriters running simultaneously
cargo run --example multiple_typewriters

# Unicode support (Chinese, Japanese, Korean, Arabic, Emoji)
cargo run --example multilingual

# Chain multiple dialogues automatically
cargo run --example chain_typewriters

# Dynamic speed control
cargo run --example dynamic_speed
```

## Dependencies

This project uses the following crates:

| Crate                                 | Version | Description |
|---------------------------------------|---------|-------------|
| [bevy](https://crates.io/crates/bevy) | 0.17.2  | Game engine |

## Contributing

Contributions are welcome!
Whether you want to fix a bug, add a feature, or improve documentation:

* Submit an **Issue** or **Pull Request**.
* Share ideas and discuss design or architecture.

## License

This project is licensed under either of

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE)
  or [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0))
* MIT license ([LICENSE-MIT](LICENSE-MIT) or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))

at your option.
