use bevy::prelude::*;

pub fn detect_changes(
    asset_server: Res<AssetServer>,
) {
    // Do you need this if settings enabled?
    let result = asset_server.watch_for_changes();
    // How to know when change occured? Read source code
}