#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use bevy::ecs::system::Resource;
use bevy::{asset::AssetServerSettings, prelude::*, winit::WinitSettings};
use bevy_dolly::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_ggrs::{GGRSPlugin, SessionType};
//use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_mod_picking::*;
use bevy_rapier3d::prelude::*;
use async_std::task;
use futures::StreamExt;
use libp2p::kad::record::store::MemoryStore;
use libp2p::kad::{GetClosestPeersError, Kademlia, KademliaConfig, KademliaEvent, QueryResult};
use libp2p::{
    development_transport, identity,
    swarm::{Swarm, SwarmEvent},
    Multiaddr, PeerId,
};
use serde::__private::de;
use std::{env, thread, error::Error, str::FromStr, time::Duration};
use futures::executor::block_on;
use crossbeam_channel::{unbounded, Sender, Receiver};

mod networks;
mod animation;
mod colliders;
mod components;
mod default_world;
mod ggrs_rollback;
mod plane_creator;
mod players;
mod systems;
mod worlds;
mod blockchain;

use animation::{animation_helper, play};
use colliders::collider;
use default_world::{create_default, menu};
use ggrs_rollback::{follow_camera, ggrs_camera, network};
use plane_creator::{db::assets, geometry::{bevy_ui, my_plane}, save::save_world};
use players::{info, movement, physics};
use worlds::{create_insight, player};
use networks::behavior::{kademlia, mdns, identify, protocol};
use networks::connection::{swarm, peers};
use networks::structs::{BackendRequest, GameRequest};

const FPS: usize = 60;
const ROLLBACK_DEFAULT: &str = "rollback_default";
const ROLLBACK_DEFAULT2: &str = "rollback_default2";
pub const WIDTH: f32 = 1200.0;
pub const HEIGHT: f32 = 800.0;

// cargo run -- --local-port 7000 --players localhost 127.0.0.1:7001
// cargo run -- --local-port 7001 --players 127.0.0.1:7000 localhost

//#[derive(Default)]
pub struct GameSender {
    pub game_sender: Sender<String>,
}

impl FromWorld for GameSender {
    fn from_world(world: &mut World) -> Self{
        let (game_sender, _) = unbounded::<String>();
        GameSender{
            game_sender,
        }
    }
}

pub struct GameReceiver {
    pub game_receiver: Receiver<String>,
}

impl FromWorld for GameReceiver {
    fn from_world(world: &mut World) -> Self{
        let (_, game_receiver) = unbounded::<String>();
        GameReceiver{
            game_receiver,
        }
    }
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let query:u32 = args[1].parse().unwrap();

    let mut local_key = identity::Keypair::from_protobuf_encoding(&peers::P2KEY).expect("Decoding Error");
    if query == 0 {
        local_key = identity::Keypair::from_protobuf_encoding(&peers::P1KEY).expect("Decoding Error");
    }
    let local_peer_id = PeerId::from(local_key.public());

    //Crossbeam channel set up
    let (networks_sender, game_receiver) = unbounded::<String>();
    let (game_sender, networks_receiver) = unbounded::<String>();
    // let my_future =
    //     protocol::core::into_protocol(private, peerid, backend_sender, backend_reciever);

    let my_future = protocol::process_swarm_events(local_key.clone(), local_peer_id, networks_sender, networks_receiver);
    thread::spawn(move || block_on(my_future).expect("Thread Spawn Error"));

    let mut app = App::new(); //.add_plugins(DefaultPlugins).run();
    //app.init_resource::<GameSender>();
    app.insert_resource(GameSender{game_sender});
    app.insert_resource(GameReceiver{game_receiver: game_receiver.clone()});
    app.init_resource::<menu::PlaneCreatorState>();
    app.add_startup_system(menu::configure_plane_creator_state);
    app.init_resource::<menu::MetaverseState>();
    app.add_startup_system(menu::configure_metaverse_state);
    //app.insert_resource(menu::MetaverseUpdate{update: 0});

    let port = network::get_port(game_receiver.clone());

    // Create a GGRS session.
    let sess_build = network::create_ggrs_session(game_receiver.clone()).unwrap();

    // Start the GGRS session.
    let sess = network::start_ggrs_session(sess_build, port, game_receiver).unwrap();

    // GGRS Configuration
    GGRSPlugin::<network::GGRSConfig>::new()
        // Define frequency of rollback game logic update.
        .with_update_frequency(FPS)
        // Define system that returns inputs given a player handle, so GGRS can send the inputs.
        .with_input_system(movement::input)
        // Register types of components and resources you want to be rolled back.
        .register_rollback_type::<Transform>()
       // .register_rollback_type::<menu::MetaverseUpdate>()
        //.register_rollback_type::<info::Velocity>()
        // These systems will be executed as part of the advance frame update.
        .with_rollback_schedule(
            Schedule::default()
                .with_stage(
                    ROLLBACK_DEFAULT,
                    SystemStage::parallel().with_system(movement::translate_player),
                )
                .with_stage_after(
                    ROLLBACK_DEFAULT,
                    ROLLBACK_DEFAULT2,
                    SystemStage::parallel().with_system(movement::animate_moving_player),
                ),
        )
        .build(&mut app);

    // GGRS Setup
    app // Add your GGRS session.
        .insert_resource(sess)
        .insert_resource(SessionType::P2PSession);

    //General Setup
    app.insert_resource(Msaa { samples: 4 })
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .insert_resource(WindowDescriptor {
            // This must come before default plugin.
            // width: 800.,
            // height: 800.,
            width: WIDTH,
            height: HEIGHT,
            title: "InsightWorld".to_owned(),
            ..Default::default()
        })
        // AssetServerSettings must be inserted before adding the AssetPlugin or DefaultPlugins.
        // Tell the asset server to watch for asset changes on disk:
        .insert_resource(AssetServerSettings {
            watch_for_changes: true,
            ..default()
        })
        // Only run the app when there is user input. This will significantly reduce CPU/GPU use.
        .insert_resource(WinitSettings::desktop_app())
        .insert_resource(bevy_ui::Images {
            img1: "default_imgs/emu.png".to_owned(),
            img2: "default_imgs/tiger.png".to_owned(),
            img3: "default_imgs/eve.png".to_owned(),
            img4: "default_imgs/fireball.png".to_owned(),
        })
        .init_resource::<bevy_ui::UiState>()
        //.init_resource::<bevy_ui::Images>()
        .init_resource::<bevy_ui::Tags>()
        .init_resource::<assets::PlaneAssets>()
        // Plugins
        .add_plugins(DefaultPlugins)
        .add_plugins(DefaultPickingPlugins)
        .add_plugin(bevy_transform_gizmo::TransformGizmoPlugin::default())
        .add_plugin(DollyCursorGrab)
        .add_plugin(EguiPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(collider::ColliderBuilderPlugin::default());

    // Camera
    app.add_startup_system(ggrs_camera::setup_camera)
        .add_system(ggrs_camera::update_camera);
    // // Follow Camera (uncomment in network.rs)
    // app.add_system(follow_camera::update_camera) //puts camera behind player
    //     .add_system(follow_camera::frame); //follows player

    // Plane Creator
    app.add_system_set(
        SystemSet::new()
            .with_run_criteria(menu::get_plane_creator_state)
            .with_system(create_default::despawn_meta.label("despawn"))
            //startup systems
            //.with_system(create_default::create_default_plane.label("setup").after("despawn"))
            .with_system(my_plane::setup_plane.label("setup").after("despawn"))
            .with_system(assets::default_assets.label("default_assets").after("despawn"))
            .with_system(bevy_ui::configure_visuals.label("config_visuals").after("despawn"))
            .with_system(bevy_ui::configure_ui_state.label("config_ui_state").after("despawn"))
            //systems
            .with_system(bevy_ui::ui_example)
            .with_system(bevy_ui::file_drop),
            //.with_system(save_world::save_scene)
            //.with_system(save_world::recreate_scene),
    );

    // Metaverse
    app.add_system_set(
        SystemSet::new()
            .with_run_criteria(menu::get_metaverse_state)
            //startup systems
            .with_system(create_default::despawn_pc.label("despawn"))
            .with_system(
                network::setup_system
                    .label("network_setup")
                    .after("despawn"),
            ) // Start p2p session and add players.
            .with_system(create_insight::create_insight_world.after("network_setup"))
            .with_system(play::setup_character.after("network_setup")) // Insert player animations.
            //systems
            .with_system(animation_helper::setup_helpers.after("network_setup")), // Find AnimationHelperSetup markers for players.
    );

    // // Menu.
    app.add_system(menu::ui_example);
   // app.add_system(menu::push_notif);
    //app.add_system(menu::check_res_changed);

    // Play stationary animations
    //  .add_system(play::play_scene);

    //egui
    // app.add_plugin(EguiPlugin)
    //     .add_plugin(WorldInspectorPlugin::new()); // Records all assets.

    app.run();

    Ok(())
}
