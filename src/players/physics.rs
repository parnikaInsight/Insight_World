use bevy::prelude::*;
use bevy::render::mesh::Indices;
use bevy::render::mesh::VertexAttributeValues;
use bevy_rapier3d::prelude::*;

use crate::players::info;

// Don't add as startup system because meshes are not done loading by that stage.
pub fn hitbox(mut commands: Commands, query: Query<Entity, With<info::Player>>) {
    // log entity components to check if already has a collider of correct dimensions, else update.
    for entity in query.iter() {
        commands.entity(entity)
            .insert(LockedAxes::ROTATION_LOCKED) 
            .insert(RigidBody::Dynamic)
            .with_children(|children| {
                children.spawn()
                    .insert(Collider::cuboid(0.5, 0.75, 0.5))
                    // Position the collider relative to the rigid-body.
                    .insert_bundle(TransformBundle::from(Transform::from_xyz(0.0, 0.0, -1.0)));
            })
            // Standard player size; update collider size if player is scaled.
            //.insert(Collider::cuboid(0.0, 1.5, 0.25)) 
            //.insert(Collider::cuboid(0.5, 0.75, 0.5))
            .insert(ColliderDebugColor(Color::hsl(220.0, 1.0, 0.3)));
    }
}

// Don't add as startup system because meshes are not done loading by that stage.
pub fn object_colliders(mut commands: Commands, query: Query<Entity, Without<info::Player>>) {
    for entity in query.iter() {
        commands.entity(entity)
            .insert(RigidBody::Dynamic)
            .insert(Collider::cuboid(0.5, 1.75, 0.25))
            .insert(ColliderDebugColor(Color::hsl(220.0, 1.0, 0.3)));
    }
}

pub fn custom_collider(
    mut commands: Commands,
    // mut ev_asset: EventReader<AssetEvent<Mesh>>,
    // mut assets: ResMut<Assets<Mesh>>,
    query: Query<(Entity, &Handle<Scene>), With<info::Player>>,
    meshes: ResMut<Assets<Mesh>>,
) {
    // for ev in ev_asset.iter() {
    //     match ev {
    //         AssetEvent::Created { handle } => {
    //             let loaded_mesh = assets.get(handle).unwrap();
    //             let mesh = &Handle<loaded_mesh>;
    //             let (vertices, indices) = get_verts_indices(meshes.get(mesh).unwrap());
    //             // commands
    //             //     .entity(entity)
    //             //     .insert(Collider::trimesh(vertices, indices));
    //         }
    //         AssetEvent::Modified { handle } => {}
    //         AssetEvent::Removed { handle } => {}
    //     }
    // }

    // // Second soln
    // for (entity, mesh) in query.iter() {
    //     println!("in custom collider");
    //     let (vertices, indices) = get_verts_indices(meshes.get(mesh).unwrap());
    //     commands
    //         .entity(entity)
    //         .insert(Collider::trimesh(vertices, indices));
    // }
}

pub fn get_verts_indices(mesh: &Mesh) -> (Vec<Vec3>, Vec<[u32; 3]>) {
    let vertices = match mesh.attribute(Mesh::ATTRIBUTE_POSITION) {
        None => panic!("Mesh does not contain vertex positions"),
        Some(vertex_values) => match &vertex_values {
            VertexAttributeValues::Float32x3(positions) => positions
                .iter()
                .map(|[x, y, z]| Vec3::new(*x, *y, *z))
                .collect(),
            _ => panic!("Unexpected types in {:?}", Mesh::ATTRIBUTE_POSITION),
        },
    };

    let indices = match mesh.indices().unwrap() {
        Indices::U16(_) => {
            panic!("expected u32 indices");
        }
        Indices::U32(indices) => indices
            .chunks(3)
            .map(|chunk| [chunk[0], chunk[1], chunk[2]])
            .collect(),
    };
    (vertices, indices)
}
