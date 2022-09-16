mod player_models;

use bevy::prelude::*;

pub fn main() {
    let mut app = App::new();

    app.insert_resource(Msaa { samples: 4 })
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .insert_resource(WindowDescriptor {
            // This must come before default plugin.
            title: "InsightWorld Systems Creator".to_string(),
            width: 800.,
            height: 800.,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins);

    app.add_startup_system(player_models::spawn::setup_my_player);
    app.run();
}