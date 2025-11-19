# bevy_ecs_typewriter

[![license](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue)](LICENSE-APACHE) <img src="https://img.shields.io/github/repo-size/Bli-AIk/bevy_ecs_typewriter.svg"/> <img src="https://img.shields.io/github/last-commit/Bli-AIk/bevy_ecs_typewriter.svg"/> <br> <img src="https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white" />

**bevy_ecs_typewriter** — 适用于 Bevy 的纯文本打字机效果插件。

| 英语                     | 简体中文 |
|------------------------|------|
| [English](./readme.md) | 简体中文 |

## 介绍

`bevy_ecs_typewriter` 是一个轻量级的 Bevy 游戏引擎打字机效果插件。
它提供纯文本管理，不依赖任何 UI 组件，使其能够灵活适配各种使用场景。

使用 `bevy_ecs_typewriter`，你可以轻松为对话、字幕或任何文本动画创建打字机效果，通过简单的组件化控制即可实现。

## 功能

* 🎮 **纯文本管理** - 不依赖 UI 组件，可与任何渲染方案配合使用
* ⚡ **简洁 API** - 提供播放、暂停、恢复和停止等简单控制接口
* 🌏 **完整 Unicode 支持** - 支持任何语言，包括中日韩文、阿拉伯文、Emoji 等
* 🔄 **多打字机支持** - 可同时运行多个打字机
* ⏱️ **动态速度控制** - 实时调整打字速度
* 📊 **进度追踪** - 获取当前进度和状态信息

## 使用方法

1. **添加到 Cargo.toml**：

   ```toml
   [dependencies]
   bevy = "0.17.2"
   bevy_ecs_typewriter = "0.0.0"
   ```

2. **将插件添加到你的应用**：

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

3. **创建打字机实体**：

   ```rust
   fn setup(mut commands: Commands) {
       let mut typewriter = Typewriter::new("你好，世界！", 0.1);
       typewriter.play();
       commands.spawn(typewriter);
   }
   ```

4. **访问当前文本**：

   ```rust
   fn display_system(query: Query<&Typewriter, Changed<Typewriter>>) {
       for typewriter in &query {
           println!("{}", typewriter.current_text);
       }
   }
   ```

## 示例

运行示例查看插件效果：

```bash
# 基础键盘控制示例
cargo run --example basic_control

# 多个打字机同时运行
cargo run --example multiple_typewriters

# Unicode 支持（中文、日文、韩文、阿拉伯文、Emoji）
cargo run --example multilingual

# 自动连续播放多段对话
cargo run --example chain_typewriters

# 动态速度控制
cargo run --example dynamic_speed
```

## 依赖

本项目使用以下 crate：

| Crate                                             | 版本    | 描述   |
| ------------------------------------------------- | ----- | ---- |
| [bevy](https://crates.io/crates/bevy) | 0.17.2 | 游戏引擎 |
| [serde](https://crates.io/crates/serde) | 1.0 | 序列化框架 |

## 贡献指南

欢迎贡献！
无论你想修复错误、添加功能或改进文档：

* 提交 **Issue** 或 **Pull Request**。
* 分享想法并讨论设计或架构。

## 许可证

本项目可依据以下任意一种许可证进行分发：

* Apache License 2.0（[LICENSE-APACHE](LICENSE-APACHE)
  或 [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0)）
* MIT License（[LICENSE-MIT](LICENSE-MIT) 或 [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT)）

可任选其一。
