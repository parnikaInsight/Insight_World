use bevy::utils::hashbrown::HashMap;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

#[derive(Debug)]
// IWorlds do must be relatively adjacent and grow in a spiral
pub struct InsightWorld{
    pub hashmap: HashMap<usize, IWorld>
}

impl InsightWorld {
    pub fn new() -> Self {
        InsightWorld{
            hashmap: HashMap::new(),
        }
    }

    pub fn add_world(&mut self, world: IWorld) {
        self.hashmap.insert(self.hashmap.len() + 1, world);
    }
}

// An IWorld is a square grid. It is comprised of levels. Every square in the grid is a location
//      possibility for a plane.
// Level 0 is the center square (index = 0). Level 1 comprises the 8 squares surrounding Level 0.
//      Level 2 are the 16 squares surrounding Level 1, which surrounds Level 0.
//      # of squares in Level i = 8i (except Level 0 has 1 square)
//      total # of squares up until Level i = (i * 2 + 1) ^ 2
// The top left square of each level starts with index 0. Index increases clockwise.
// Planes in a world do not have to be connected to each other. The outermost edges of the IWorld
//      form a bordering square boundary.
//  -----
//  | 00|     0 denotes an IPlane
//  |00 |
//  |  0|
//  -----
#[derive(Debug)]
pub struct IWorld{
    hashmap: HashMap<(u32, u32), IPlane>,
} 

impl IWorld {
    pub fn new() -> Self {
        IWorld{
            hashmap: HashMap::new(),
        }
    }

    pub fn add_plane(
        &mut self, 
        planes: Vec<&IPlane>, 
        mut commands: Commands, 
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
    ) {
        let mut index = 0.0;
        for plane in planes {
            // IWorld hashmap
            self.hashmap.insert((plane.level, plane.index), *plane);

            // Plane transform calculations
            let mut trans_x: i32 = 0;
            let mut trans_y: i32  = 0;
            let mut trans_z: i32 = 0;
            let top_row = insert_into_vec(0, 2 * plane.level + 1);
            let right_col = insert_into_vec(2 * plane.level + 1, 4 * 
                    plane.level + 1);
            let bottom_row = insert_into_vec(4 * plane.level + 1, 6 * 
                    plane.level + 1);
            let mut left_col = insert_into_vec(6 * plane.level + 1, 6 * plane.level + 
                    1 + plane.level);
            if plane.level == 1{
                left_col.pop();
            }
            if top_row.contains(&plane.index) {
                trans_z = -15 * (plane.level as i32);
                trans_y = 0;
                trans_x = -15 * (plane.level as i32) + 15 * (plane.index as i32);
            }
            if right_col.contains(&plane.index) {
                trans_x = 15 * (plane.level as i32);
                trans_y = 0;
                trans_z = -15 * ((plane.level as i32) - 1) + 15 * ((plane.index as i32) - 2 * 
                    (plane.level as i32) - 1);
            }
            if bottom_row.contains(&plane.index) {
                trans_z = 15 * (plane.level as i32);
                trans_y = 0;
                trans_x = 15 * ((plane.level as i32) - 1) - 15 * ((plane.index as i32) - 4 * 
                    (plane.level as i32) - 1);
            }
            if left_col.contains(&plane.index) {
                trans_x = -15 * (plane.level as i32);
                trans_y = 0;
                let pl = plane.level as i32;
                trans_z = 15 * (pl - 1) - 15 * ((plane.index as i32) - 6 * pl - 1);
            }
            let mut trans = Transform::from_xyz(0.0, 0.0, 0.0);
            if plane.level != 0 {
                trans = Transform::from_xyz(trans_x as f32, trans_y as f32, trans_z as f32);
            }

            // Plane
            commands
                .spawn_bundle(PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Plane { size: 15.0 })), //PLANE_SIZE
                    material: materials.add(Color::rgb(0.1 * index, 0.1 * index, 0.1 * index).into()),
                    transform: trans,
                    ..Default::default()
                })
                .insert(RigidBody::Fixed)
                //half the cube size
                .insert(Collider::cuboid(7.5, 7.5, 7.5)) 
                .insert(ColliderDebugColor(
                    Color::hsl(220.0, 1.0, 0.3)
                ));

            // Light
            commands.spawn_bundle(PointLightBundle {
                transform: Transform::from_xyz(4.0, 18.0, 14.0),
                ..Default::default()
            });
            index += 1.0;
        }
    }
}

// IPlanes are not limited to one IWorld, so no worldid parameter is required.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct IPlane {
   // pub gltfs: Vec<IGltf>,
    pub level: u32, 
    pub index: u32, // ranges from 0 to (8 * level - 1)
}

// impl IPlane {
//     pub fn add_gltf() -> Self {}
// }

// #[derive(Debug)]
// pub struct IGltf {
//     pub name: String,
//     pub scene: String,
//     pub animation: String,
// }

pub fn insert_into_vec(start: u32, end: u32) -> Vec<u32> {
    let mut vec = Vec::new();
    for i in start..end {
        vec.push(i);
    }
    vec
}