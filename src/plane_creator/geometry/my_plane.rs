use bevy::prelude::*;

//use crate::plane_creator::db::db_worlds;
use crate::db::db_assets;

// set up a simple 3D scene
pub fn setup_plane(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            ..Default::default()
        })
        .insert_bundle(bevy_mod_picking::PickableBundle::default())
        .insert(bevy_transform_gizmo::GizmoTransformable);
    //db_worlds::put("plane".to_string(), db_worlds::transform_to_string(Transform::default()));

    // mini cube
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 0.5 })),
            transform: Transform::from_xyz(1.0, 0.5, 0.0),
            ..Default::default()
        })
        .insert_bundle(bevy_mod_picking::PickableBundle::default())
        .insert(bevy_transform_gizmo::GizmoTransformable);
    //db_worlds::put("mini_cube".to_string(), db_worlds::transform_to_string(Transform::from_xyz(1.0, 0.5, 0.0)));

    // Load gltf.
    let player_handle1: Handle<Scene> = asset_server.load("test/pool_ball.glb#Scene0");
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 0.5 })),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..Default::default()
        })
        .insert_bundle(bevy_mod_picking::PickableBundle::default())
        .insert(bevy_transform_gizmo::GizmoTransformable)
        .with_children(|children| {
            children.spawn_bundle(SceneBundle {
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, 0.0),
                    scale: Vec3::new(0.5, 0.5, 0.5),
                    ..default()
                },
                scene: player_handle1.clone(),
                ..default()
            });
            // .insert(Collider::cuboid(half_size.x, half_size.y, half_size.z))
            // .insert(ColliderMassProperties::Density(density));
        });
    // db_worlds::put("pool_ball".to_string(), db_worlds::transform_to_string(Transform {
    //     translation: Vec3::new(0.0, 0.0, 0.0),
    //     scale: Vec3::new(0.5, 0.5, 0.5),
    //     ..default()
    // }));

    // light
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });
}

pub fn add_block(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    keys: Res<Input<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::B) {
        commands
            .spawn_bundle(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
                material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
                transform: Transform::from_xyz(0.0, 0.5, 0.0),
                ..Default::default()
            })
            .insert_bundle(bevy_mod_picking::PickableBundle::default())
            .insert(bevy_transform_gizmo::GizmoTransformable);
    }
}
