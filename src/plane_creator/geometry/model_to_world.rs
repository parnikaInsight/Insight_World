use bevy::sprite::collide_aabb::Collision;
use bevy::{gltf::Gltf, prelude::*, render::primitives::Aabb, sprite::collide_aabb};
use glam::f32::Vec3A;

// with aabb
pub fn sizer(
    mut ass: ResMut<Assets<Scene>>,
    asset_server: Res<AssetServer>,
    mut visible_aabb_query: Query<(Entity, &Aabb, &GlobalTransform)>,
) {
    visible_aabb_query.par_for_each_mut(1024, |(entity, model_aabb, transform)| {
        let model = transform.compute_matrix(); // model
        let world_center = model.transform_point3(Vec3::from(model_aabb.center)); // center of aabb in world space
        let world_half_extents = model.transform_point3(Vec3::from(model_aabb.half_extents)); // half-extents of aabb in world space
        println!("world_center: {:?}", world_center);
        println!("world_half_extents: {:?}", world_half_extents);
    });
}

pub fn get_collision_params(collision_box: Aabb) -> (Vec3, Vec3) {
    let pos = collision_box.center;
    let half_extents = collision_box.half_extents.to_array();
    let width = half_extents[0]; //* 2.0;
    let height = half_extents[1]; //* 2.0;
    let length = half_extents[2]; // * 2.0;
    let dim: Vec3 = Vec3 {
        x: width,
        y: height,
        z: length,
    };
    (Vec3::from(pos), dim)
}

pub fn intersect(a_pos: Vec3, a_dim: Vec3, b_pos: Vec3, b_dim: Vec3) -> bool {
    let min_aX = a_pos.x - a_dim.x;
    let max_aX = a_pos.x + a_dim.x;
    let min_aY = a_pos.y - a_dim.y;
    let max_aY = a_pos.y + a_dim.y;
    let min_aZ = a_pos.z - a_dim.z;
    let max_aZ = a_pos.z + a_dim.z;

    let min_bX = b_pos.x - b_dim.x;
    let max_bX = b_pos.x + b_dim.x;
    let min_bY = b_pos.y - b_dim.y;
    let max_bY = b_pos.y + b_dim.y;
    let min_bZ = b_pos.z - b_dim.z;
    let max_bZ = b_pos.z + b_dim.z;

    return (min_aX <= max_bX
        && max_aX >= min_bX
        && min_aY <= max_bY
        && max_aY >= min_bY
        && min_aZ <= max_bZ
        && max_aZ >= min_bZ);
}

