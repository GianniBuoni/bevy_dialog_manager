use bevy::prelude::*;
use bevy_dialog_manager::prelude::*;

fn main() {
    let mut app = App::new();
    // add plugins
    app.add_plugins(DefaultPlugins);
    app.add_plugins(DialogManagerPlugin.build());
    // init resources
    app.init_resource::<RCurrentDialog>();
    // run systems
    app.add_systems(Startup, init);
    app.add_systems(Update, print_current_dialog);
    app.run();
}

#[derive(Default, Resource)]
struct RCurrentDialog {
    handle: Option<Handle<DialogScript>>,
}

fn init(
    asset_server: Res<AssetServer>,
    mut current_dialog: ResMut<RCurrentDialog>,
) {
    let asset_handle = asset_server.load::<DialogScript>("dialog/simple.toml");
    current_dialog.handle = Some(asset_handle.clone());

    info!("Dialog asset: {asset_handle:?} loaded!");
}

fn print_current_dialog(
    asset_server: Res<Assets<DialogScript>>,
    current_dialog: Res<RCurrentDialog>,
) {
    let not_ready = || {
        info!("Dialog asset not yet ready!");
    };
    let Some(handle) = current_dialog.handle.as_ref() else {
        not_ready();
        return;
    };
    let Some(script) = asset_server.get(handle) else {
        not_ready();
        return;
    };
    once!({ dbg!(script) })
}
