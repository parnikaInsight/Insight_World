use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(
            0xF9 as f32 / 255.0,
            0xF9 as f32 / 255.0,
            0xFF as f32 / 255.0,
        )))
        .insert_resource(Msaa::default())
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_startup_system(setup_graphics)
        .add_startup_system(setup_physics)
        .add_system_to_stage(CoreStage::PostUpdate, display_events)
        .run();
}

fn setup_graphics(mut commands: Commands) {
    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 25.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
}

fn display_events(
    mut collision_events: EventReader<CollisionEvent>,
    mut contact_force_events: EventReader<ContactForceEvent>,
) {
    for collision_event in collision_events.iter() {
        println!("Received collision event: {:?}", collision_event);
    }

    for contact_force_event in contact_force_events.iter() {
        println!("Received contact force event: {:?}", contact_force_event);
    }
}

pub fn setup_physics(mut commands: Commands) {
    /*
     * Ground
     */
    commands
        .spawn_bundle(TransformBundle::from(Transform::from_xyz(0.0, -1.2, 0.0)))
        .insert(Collider::cuboid(4.0, 1.0, 1.0));

    commands
        .spawn_bundle(TransformBundle::from(Transform::from_xyz(0.0, 5.0, 0.0)))
        .insert(Collider::cuboid(4.0, 1.5, 1.0))
        .insert(Sensor);

    commands
        .spawn_bundle(TransformBundle::from(Transform::from_xyz(0.0, 13.0, 0.0)))
        .insert(RigidBody::Dynamic)
        .insert(Collider::cuboid(0.5, 0.5, 0.5))
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(ContactForceEventThreshold(30.0));
}
