use bevy::prelude::*;
use bevy_dialog_manager::prelude::*;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_plugins(DialogManagerPlugin.build());
    app.add_systems(Startup, hello_world);
    app.run();
}

fn hello_world(asset_server: Res<AssetServer>) {
    let _asset = asset_server.load::<DialogScript>("dialog/simple.toml");
    info!("Hello from Bevy!");
}
