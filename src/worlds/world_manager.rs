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
// The x-z bevy grid maps to an x-y grid for plane coordinates. Y coordinate stays the same.
// Planes in a world do not have to be connected to each other. The outermost edges of the IWorld
//      form a bordering square boundary.
//  -----
//  | 00|     0 denotes an IPlane
//  |00 |
//  |  0|
//  -----
#[derive(Debug)]
pub struct IWorld{
    hashmap: HashMap<(u32, i32, u32), IPlane>,
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
            self.hashmap.insert((plane.x, plane.y, plane.z), *plane);

            // Plane transform calculations
            let mut trans_x: i32 = plane.x as i32;
            let trans_y: i32  = plane.y as i32;
            let mut trans_z: i32 = plane.z as i32;
            trans_x = 15 * trans_x;
            trans_z = -15 * trans_z;
            let trans = Transform::from_xyz(
                trans_x as f32, trans_y as f32, trans_z as f32
            );

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
    pub x: u32, 
    pub y: i32,
    pub z: u32,
}

impl IPlane {
    pub fn new(x: u32, y: i32, z: u32) -> IPlane {
        Self {
            x, y, z,
        }
    }
    pub fn get_level(&self) -> u32 {
        let mut level = 0;
        if self.x > level {
            level = self.x;
        }
        if self.z > level {
            level = self.z;
        }
        level
    }
}

// #[derive(Debug)]
// pub struct IGltf {
//     pub name: String,
//     pub scene: String,
//     pub animation: String,
// }

// Boundary: Check if square has an adjacent square. 
// Any side with no adjacent square is part of the boundary.

