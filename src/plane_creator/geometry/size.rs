use bevy::sprite::collide_aabb::Collision;
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
    match ass.get_mut(&asset_server.load("default_gltfs/emu.glb#Scene0")) {
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

pub fn get_collision_params(collision_box: Aabb) -> (Vec3, Vec3) {
    let pos = collision_box.center;
    let half_extents = collision_box.half_extents.to_array();
    let width = half_extents[0];//* 2.0;
    let height = half_extents[1];//* 2.0;
    let length = half_extents[2];// * 2.0;
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

pub fn detect_collisions(
    mut ass: ResMut<Assets<Scene>>,
    asset_server: Res<AssetServer>,
) -> (i32, i32, i32, i32, i32, i32) {
    let mut index = 0;
    let (vect, len) = sizer(ass, asset_server);
    let mut vec = vect.clone();
    let mut count = 0;
    let base = 2 as i32;
    let mut min_x = 1000000000;
    let mut max_x = -1000000000;
    let mut min_y = 1000000000;
    let mut max_y = -1000000000;
    let mut min_z = 1000000000;
    let mut max_z = -1000000000;
    if len > 0 {
        while index <= len - 1 {
            if let Some(popped) = vec.pop() {
                let vec2 = vec;
                vec = vec2.clone();
                let (comp_pos, comp_dim) = get_collision_params(popped);
                let min_aX = comp_pos.x - comp_dim.x;
                if (min_aX.floor() as i32) < min_x {
                    min_x = min_aX.floor() as i32;
                }
                let max_aX = comp_pos.x + comp_dim.x;
                if (max_aX.floor() as i32) > max_x {
                    max_x = max_aX.floor() as i32;
                }
                let min_aY = comp_pos.y - comp_dim.y;
                if (min_aY.floor() as i32) < min_y {
                    min_y = min_aY.floor() as i32;
                }
                let max_aY = comp_pos.y + comp_dim.y;
                if (max_aY.floor() as i32) > max_y {
                    max_y = max_aY.floor() as i32;
                }
                let min_aZ = comp_pos.z - comp_dim.z;
                if (min_aZ.floor() as i32) < min_z {
                    min_z = min_aZ.floor() as i32;
                }
                let max_aZ = comp_pos.z + comp_dim.z;
                if (max_aZ.floor() as i32) > max_z {
                    max_z = max_aZ.floor() as i32;
                }
                for i in vec2 {
                    let (iter_pos, iter_dim) = get_collision_params(i);
                    if intersect(comp_pos, comp_dim, iter_pos, iter_dim) {
                        count += 1;
                    }

                    // if let Some(collision) =
                    //     bevy::sprite::collide_aabb::collide(comp_pos, comp_dim, iter_pos, iter_dim)
                    // {
                    //     match collision {
                    //         Collision::Inside => count += 1,
                    //         _ => (),
                    //     }
                    // }
                }
                index += 1;
            }
        }
    }
    println!("collisions: {}", count);
    println!(
        "min x: {}, max x: {}, min y: {}, max y: {}, min z: {}, max z: {}",
        min_x, max_x, min_y, max_y, min_z, max_z
    );
    (min_x, max_x, min_y, max_y, min_z, max_z)
}

pub fn scale_for_spawn(
    mut ass: ResMut<Assets<Scene>>, 
    asset_server: Res<AssetServer>, 
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let player_handle1: Handle<Scene> = asset_server.load("default_gltfs/emu.glb#Scene0");
    let (min_x, max_x, min_y, max_y, min_z, max_z) = detect_collisions(ass, asset_server);
    
    let x = max_x - min_x;
    let y = max_y - min_y;
    let z = max_z - min_z;

    let mut min = x;
    if y < min {
        min = y;
    }
    if z < min {
        min = z;
    }
    println!("min {}", min);
    
    let mut max = x;
    if y > max {
        max = y;
    }
    if z > max {
        max = z;
    }
    println!("max {}", max);

    min = max;

    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 0.5 })),
            material: materials.add(StandardMaterial {
                base_color: Color::rgba(0.2, 0.7, 0.1, 0.0),
                alpha_mode: AlphaMode::Mask(0.5),
                ..default()
            }),
            // transform: Transform::from_xyz(0.0, 0.0, 0.0),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.0),
                // scale: Vec3::new(10.0, 10.0, 10.0),
                ..default()
            },
            ..Default::default()
        })
        .insert_bundle(bevy_mod_picking::PickableBundle::default())
        .insert(bevy_transform_gizmo::GizmoTransformable)
        .with_children(|children| {
            children.spawn_bundle(SceneBundle {
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, 0.0),
                    //scale: Vec3::new(1.0/ (min as f32), 1.0/ (min as f32), 1.0/ (min as f32)),
                    //scale: Vec3::new(0.01/ (min as f32), 0.01/ (min as f32), 0.01/ (min as f32)),
                    ..default()
                },
                scene: player_handle1.clone(),
                ..default()
            });
        });
        println!("vec {}", Vec3::new(1.0/ (min as f32), 1.0/ (min as f32), 1.0/ (min as f32)))
}
