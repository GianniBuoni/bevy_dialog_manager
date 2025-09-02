# 🐦 Dialog manager for Bevy

`bevy_dialog_manger` aims to provide a file based loading and handling for dialog trees in Bevy games, while taking advantage of the Bevy asset loader and Rust's type system to provide a (hopefuly) ergonomic api.

# 🥅 Goals

- 📁 Dialog trees defined through text files (TOML)
- 🧠 Headless: dialog manager should handle providing dialog to a game, but how that dialog is displayed/animated/rendered is beyond the scope of this plugin.
- 👀 Dialog visulization: get a visual representation of a dialog tree.
- 🔍 Linting: parse dialogs (in-tree and globally) to warn user aboult invalid or unused data/types.

# TODO

- [ ] Define node types/builders
- [ ] TOML serialization
- [ ] Implement bevy's `AssetLoader`
