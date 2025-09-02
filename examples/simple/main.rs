use bevy::prelude::*;
use bevy_dialog_manager::prelude::*;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_plugins(DialogManagerPlugin.build());
    app.add_systems(Startup, hello_world);
    app.run();
}

fn hello_world() {
    info!("Hello from Bevy!");
}
