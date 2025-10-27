//! This examle parses a smple set of dialog nodes
//! Then creates a resoruce from the current dialog handle.

use bevy::prelude::*;
use bevy_dialog_manager::prelude::*;

fn main() {
    let mut app = App::new();
    // add plugins
    app.add_plugins(DefaultPlugins);
    app.add_plugins(DialogManagerPlugin.build());
    // run systems
    app.add_systems(Startup, init);
    app.add_systems(Update, print_current_dialog);
    app.run();
}

fn init(
    asset_server: Res<AssetServer>,
    mut current_dialog: ResMut<CurrentDialogNode>,
) {
    let asset_handle = asset_server.load::<DialogScript>("dialog/simple.toml");
    current_dialog.dialog_handle = Some(asset_handle.clone());

    info!("Dialog asset: {asset_handle:?} loaded!");
}

fn print_current_dialog(
    asset_server: Res<Assets<DialogScript>>,
    current_dialog: Res<CurrentDialogNode>,
) {
    let not_ready = || {
        info!("Dialog asset not yet ready!");
    };
    let Some(handle) = current_dialog.dialog_handle.as_ref() else {
        not_ready();
        return;
    };
    let Some(script) = asset_server.get(handle) else {
        not_ready();
        return;
    };
    once!({
        dbg!(current_dialog);
        dbg!(script)
    })
}
