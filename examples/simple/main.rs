//! This examle parses a sample set of dialog nodes,
//! then creates a resoruce from the current dialog handle.

use bevy::prelude::*;
use bevy_dialog_manager::prelude::*;

fn main() {
    let mut app = App::new();
    // add plugins
    app.add_plugins(DefaultPlugins);
    app.add_plugins(DialogManagerPlugin.build());
    // init res
    app.init_resource::<RAppReady>();
    // run systems
    app.add_systems(Startup, load_assets);
    app.add_systems(
        Update,
        (
            init_resources.run_if(assets_ready),
            print_current_dialog.run_if(app_ready),
        ),
    );
    app.run();
}

#[derive(Resource, Default)]
struct RAppReady {
    assets_loaded: bool,
    resources_loaded: bool,
}

/// Systems to check state of startup
fn assets_ready(app_ready: Res<RAppReady>) -> bool {
    app_ready.assets_loaded && !app_ready.resources_loaded
}

fn app_ready(app_ready: Res<RAppReady>) -> bool {
    app_ready.assets_loaded && app_ready.resources_loaded
}

/// Generalized warning if any of the checks fails.
fn not_ready() {
    warn!(
        "Attempted to init resources before assets were loaded; check function orders/scheduling!"
    )
}

/// Assets loaded first and the asset handle is registered to the resoursce
fn load_assets(
    asset_server: Res<AssetServer>,
    mut current_dialog: ResMut<CurrentDialogNode>,
    mut app_ready: ResMut<RAppReady>,
) {
    let asset_handle = asset_server.load::<DialogScript>("dialog/simple.toml");
    current_dialog.dialog_handle = Some(asset_handle.clone());
    app_ready.assets_loaded = true;

    info!("Dialog asset: {asset_handle:?} loaded!")
}

/// Initializing resources happen on update; was running into issues where
/// asset server was empty when this was chained after
/// `load_assets()` at Startup.
fn init_resources(
    asset_server: Res<Assets<DialogScript>>,
    mut app_ready: ResMut<RAppReady>,
    mut current_dialog: ResMut<CurrentDialogNode>,
) {
    if app_ready.assets_loaded {
        if let Err(e) = current_dialog.get_script_nodes(asset_server) {
            not_ready();
            error!("{e}");
            return;
        }
        app_ready.resources_loaded = true;
        info!("Current dialog resource: loaded!")
    } else {
        error!("Asset server was not loaded!")
    }
}

fn print_current_dialog(
    asset_server: Res<Assets<DialogScript>>,
    current_dialog: Res<CurrentDialogNode>,
) {
    let id = match current_dialog.get_handle() {
        Ok(id) => id,
        Err(e) => {
            not_ready();
            error!("{e}");
            return;
        }
    };
    let Some(script) = asset_server.get(&id) else {
        not_ready();
        return;
    };
    once!({
        dbg!(current_dialog);
        dbg!(script)
    })
}
