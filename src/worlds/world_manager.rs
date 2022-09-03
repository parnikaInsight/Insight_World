use bevy::prelude::*;
use bevy::utils::hashbrown::HashMap;
use bevy_rapier3d::prelude::*;

#[derive(Debug)]
// IWorlds do must be relatively adjacent and grow in a spiral
pub struct InsightWorld {
    pub hashmap: HashMap<usize, IWorld>,
}

impl InsightWorld {
    pub fn new() -> Self {
        InsightWorld {
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
// Coordinates follow bevy grid system.
// Planes in a world do not have to be connected to each other. The outermost edges of the IWorld
//      form a bordering square boundary.
//  -----
//  | 00|     0 denotes an IPlane
//  |00 |
//  |  0|
//  -----
#[derive(Debug)]
pub struct IWorld {
    hashmap: HashMap<(i32, i32, i32), IPlane>,
}

impl IWorld {
    pub fn new() -> Self {
        IWorld {
            hashmap: HashMap::new(),
        }
    }

    // Boundary: Surrounds outermost planes
    pub fn get_boundary(&mut self, mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
        let mut x_min = 0;
        let mut x_max = 0;
        let mut y_min = 0;
        let mut y_max = 0;
        let mut z_min = 0;
        let mut z_max = 0;
        for (x, y, z) in self.hashmap.keys() {
            if *x < x_min {
                x_min = *x;
            }
            if *x > x_max {
                x_max = *x;
            }
            if *y < y_min {
                y_min = *y;
            }
            if *y > y_max {
                y_max = *y;
            }
            if *z < z_min {
                z_min = *z;
            }
            if *z > z_max {
                z_max = *z;
            }
        }

        let x_half = (15 * (x_min + x_max) / 2) as f32;
        let y_half = (15 * (y_min + y_max) / 2) as f32;
        let z_half = (15 * (z_min + z_max) / 2) as f32;

        let x_half_dist = (15 * (x_max - x_min) / 2) as f32;
        let y_half_dist = (15 * (y_max - y_min) / 2) as f32;
        let z_half_dist = (15 * (z_max - z_min) / 2) as f32;

        let transform = Transform::from_xyz(
            x_half, 
            y_half, 
            z_half 
        );
        // Plane
        commands
            .spawn_bundle(PbrBundle {
                transform,
                ..Default::default()
            })
            .insert(RigidBody::Fixed) 
            //half the cube size
            .insert(Collider::cuboid( // Should player be able to fall off plane?
                x_half_dist + 7.5,
                y_half_dist, 
                z_half_dist + 7.5
            ))
            .insert(ColliderDebugColor(
                Color::hsl(220.0, 1.0, 0.3))
            );

        commands.spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 0.5 })),
            transform: Transform {
                translation: Vec3::new(0.0, 3.0, 0.0),
                ..default()
            },
            ..Default::default()
        });
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
            let trans_y: i32 = plane.y as i32;
            let mut trans_z: i32 = plane.z as i32;
            trans_x = 15 * trans_x;
            trans_z = 15 * trans_z;
            let trans = Transform::from_xyz(trans_x as f32, trans_y as f32, trans_z as f32);

            // Plane
            commands
                .spawn_bundle(PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Plane { size: 15.0 })), //PLANE_SIZE
                    material: materials
                        .add(Color::rgb(0.1 * index, 0.1 * index, 0.1 * index).into()),
                    transform: trans,
                    ..Default::default()
                })
                .insert(RigidBody::Fixed)
                //half the cube size
                .insert(Collider::cuboid(7.5, 0.0, 7.5))
                .insert(ColliderDebugColor(Color::hsl(220.0, 1.0, 0.3)));

            // Light
            commands.spawn_bundle(PointLightBundle {
                transform: Transform::from_xyz(4.0, 18.0, 14.0),
                ..Default::default()
            });
            index += 1.0;
        }
        self.get_boundary(commands, meshes);
    }
}

// IPlanes are not limited to one IWorld, so no worldid parameter is required.
// Coordinates as per bevy grid coordinates
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct IPlane {
    // pub gltfs: Vec<IGltf>,
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl IPlane {
    pub fn new(x: i32, y: i32, z: i32) -> IPlane {
        Self { x, y, z }
    }
    pub fn get_level(&self) -> u32 {
        let mut level = 0;
        if i32::abs(self.x) > level {
            level = self.x;
        }
        if i32::abs(self.z) > level {
            level = self.z;
        }
        level as u32
    }
}

// #[derive(Debug)]
// pub struct IGltf {
//     pub name: String,
//     pub scene: String,
//     pub animation: String,
// }
