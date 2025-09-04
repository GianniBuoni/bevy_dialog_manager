# 🐦 Dialog manager for Bevy

`bevy_dialog_manger` aims to handle file based loading for dialog trees in games using Bevy ECS. Taking advantage of the Bevy asset loader and Rust's type system, `beyv_dialog_manager` will provide an ergonomic API for use in the game, as well as a simple format in which to write the script.

## 🥅 Goals

- 📁 Dialog trees defined through text files (TOML)
- 🧠 Headless: dialog manager should handle providing dialog to a game, but how that dialog is displayed/animated/rendered is beyond the scope of this plugin.
- 👀 Dialog visulization: get a visual representation of a dialog tree.
- 🔍 Linting: parse dialogs (in-tree and globally) to warn user aboult invalid or unused data/types.
- 💬 Translation: provide a system for localizing lines of text separate from the script files.

## TODO

- [ ] Define node types/builders
- [ ] TOML serialization
- [ ] Implement bevy's `AssetLoader`
