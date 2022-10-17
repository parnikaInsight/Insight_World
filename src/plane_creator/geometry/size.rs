use bevy::{gltf::Gltf, prelude::*, render::primitives::Aabb, sprite::collide_aabb};
use glam::f32::Vec3A;

// with aabb
pub fn sizer(
    mut ass: ResMut<Assets<Scene>>,
    asset_server: Res<AssetServer>,
    //as_mesh: ResMut<Assets<Mesh>>,
) -> (Vec<Aabb>, i32) {
    let mut v = Vec::new();
    let mut count = 0;
    match ass.get_mut(&asset_server.load("default_gltfs/victorian_street_lamp.glb#Scene0")) {
        Some(res) => {
            let mut query_one = res.world.query::<(&Aabb)>();
            for c in query_one.iter(&res.world) {
                println!("{:?}", c);
                v.push(c.clone());
                count += 1;
            }
            println!("aabb {}", count);
            (v, count)
        }
        None => {
            println!("hello");
            (v, count)
        }
    }
}

//with meshes
pub fn sizer2(ass: Res<Assets<Gltf>>, asset_server: Res<AssetServer>, as_mesh: Res<Assets<Mesh>>) {
    match ass.get(&asset_server.load("default_gltfs/victorian_street_lamp.glb")) {
        Some(res) => {
            let meshes = res.meshes.clone();
            let mut count = 0;
            for c in meshes.iter() {
                println!("{:?}", c);
                count += 1;
            }
            println!("meshes {}", count);
        }
        None => println!("2 hello"),
    }
}

pub fn get_collision_params(collision_box: Aabb) -> (Vec3, Vec2) {
    let pos = collision_box.center;
    let half_extents = collision_box.half_extents.to_array();
    let width = half_extents[0] * 2.0;
    let height = half_extents[1] * 2.0;
    let dim: Vec2 = Vec2 {
        x: width,
        y: height,
    };
    (Vec3::from(pos), dim)
}

pub fn detect_collisions(mut ass: ResMut<Assets<Scene>>, asset_server: Res<AssetServer>) {
    let mut index = 0;
    let (vect, len) = sizer(ass, asset_server);
    let mut vec = vect.clone();
    let mut count = 0;
    if len > 0 {
        while index < len - 1 {
            if let Some(popped) = vec.pop() {
                let vec2 = vec;
                vec = vec2.clone();
                let (comp_pos, comp_dim) = get_collision_params(popped);
                for i in vec2 {
                    let (iter_pos, iter_dim) = get_collision_params(i);
                    if let Some(collision) =
                        bevy::sprite::collide_aabb::collide(comp_pos, comp_dim, iter_pos, iter_dim)
                    {
                        count += 1;
                    }
                }
                index += 1;
            }
        }
    }
    println!("collisions: {}", count);
}
