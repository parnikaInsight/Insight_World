// NOTE: this demo is great for debugging despawning.
//       It was extracted for one of the debug branch from @audunhalland
//       in https://github.com/dimforge/bevy_rapier/issues/75

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

fn main() {
    App::new()
        .init_resource::<Game>()
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .insert_resource(Msaa::default())
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_game)
        .add_system(cube_sleep_detection)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        .run();
}

#[derive(Default)]
struct Stats {
    generated_blocks: i32,
    cleared_blocks: i32,
    lost_blocks: i32,
    lost_cube: bool,
}

impl Stats {
    fn health(&self) -> f32 {
        if self.lost_cube {
            0.0
        } else if self.cleared_blocks == 0 {
            if self.lost_blocks > 0 {
                0.0
            } else {
                1.0
            }
        } else {
            let lost_ratio = self.lost_blocks as f32 / self.cleared_blocks as f32;

            1.0 - lost_ratio
        }
    }
}

struct Game {
    n_lanes: usize,
    n_rows: usize,
    stats: Stats,
    cube_colors: Vec<Color>,
    current_cube_joints: Vec<Entity>,
}

impl Game {
    fn floor_y(&self) -> f32 {
        -(self.n_rows as f32) * 0.5
    }
    fn left_wall_x(&self) -> f32 {
        -(self.n_lanes as f32) * 0.5
    }
}

impl Default for Game {
    fn default() -> Self {
        Self {
            n_lanes: 10,
            n_rows: 20,
            stats: Stats::default(),
            cube_colors: vec![],
            current_cube_joints: vec![],
        }
    }
}

fn byte_rgb(r: u8, g: u8, b: u8) -> Color {
    Color::rgb(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0)
}

fn setup_game(mut commands: Commands, mut game: ResMut<Game>) {
    game.cube_colors = vec![
        byte_rgb(0, 244, 243),
        byte_rgb(238, 243, 0),
        byte_rgb(177, 0, 254),
        byte_rgb(27, 0, 250),
        byte_rgb(252, 157, 0),
        byte_rgb(0, 247, 0),
        byte_rgb(255, 0, 0),
    ];

    commands.spawn().insert_bundle(Camera2dBundle::default());

    setup_board(&mut commands, &*game);

    // initial cube
    spawn_cube(&mut commands, &mut game);
}

#[derive(Clone, Copy, Debug)]
enum CubeKind {
    I,
}

impl CubeKind {
    fn random() -> Self {
        Self::I
    }

    fn layout(&self) -> CubeLayout {
        CubeLayout {
            coords: [(1, 1), (1, 0), (1, -1), (1, -2)],
            joints: vec![(0, 1), (1, 2), (2, 3)],
        }
    }
}

struct CubeLayout {
    coords: [(i32, i32); 4],
    joints: Vec<(usize, usize)>,
}

#[derive(Component)]
struct Block;

fn setup_board(commands: &mut Commands, game: &Game) {
    let floor_y = game.floor_y();

    // Add floor
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.5, 0.5, 0.5),
                custom_size: Some(Vec2::new(game.n_lanes as f32 * 30.0, 60.0)),
                ..Default::default()
            },
            transform: Transform::from_xyz(0.0, floor_y - 30.0 * 0.5, 0.0),
            ..Default::default()
        })
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(
            game.n_lanes as f32 * 30.0 / 2.0,
            60.0 / 2.0,
        ));
}

fn spawn_cube(commands: &mut Commands, game: &mut Game) {
    let kind = CubeKind::random();
    let CubeLayout { coords, joints } = kind.layout();

    let block_entities: Vec<Entity> = coords
        .iter()
        .map(|(x, y)| {
            let lane = (game.n_lanes as i32 / 2) - 1 + x;
            let row = game.n_rows as i32 - 1 + y;
            spawn_block(commands, game, kind, lane, row)
        })
        .collect();

    game.current_cube_joints.clear();
    for (i, j) in &joints {
        let x_dir = coords[*j].0 as f32 - coords[*i].0 as f32;
        let y_dir = coords[*j].1 as f32 - coords[*i].1 as f32;

        let anchor_1 = Vec2::new(x_dir * 0.5, y_dir * 0.5);
        let anchor_2 = Vec2::new(x_dir * -0.5, y_dir * -0.5);

        commands
            .entity(block_entities[*j])
            .with_children(|children| {
                let id = children
                    .spawn()
                    .insert(ImpulseJoint::new(
                        block_entities[*i],
                        RevoluteJointBuilder::new()
                            .local_anchor1(anchor_1)
                            .local_anchor2(anchor_2)
                            .build(),
                    ))
                    .id();
                game.current_cube_joints.push(id);
            });
    }

    game.stats.generated_blocks += block_entities.len() as i32;
}

fn spawn_block(
    commands: &mut Commands,
    game: &Game,
    kind: CubeKind,
    lane: i32,
    row: i32,
) -> Entity {
    // x, y is the center of the block
    let x = game.left_wall_x() + lane as f32 + 0.5;
    let y = game.floor_y() + row as f32 + 0.5;

    // Game gets more difficult when this is lower:
    let linear_damping = 3.0;

    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: game.cube_colors[kind as usize],
                custom_size: Some(Vec2::new(30.0, 30.0)),
                ..Default::default()
            },
            transform: Transform::from_xyz(x, y, 0.0),
            ..Default::default()
        })
        .insert(RigidBody::Dynamic)
        .insert(Damping {
            linear_damping,
            angular_damping: 0.0,
        })
        .insert(Collider::cuboid(30.0 / 2.0, 30.0 / 2.0))
        .insert(Block)
        .id()
}

fn cube_sleep_detection(
    mut commands: Commands,
    mut game: ResMut<Game>,
    block_query: Query<(Entity, &GlobalTransform)>,
) {
    let all_blocks_sleeping = true;

    if all_blocks_sleeping {
        for joint in &game.current_cube_joints {
            commands.entity(*joint).despawn();
        }

        clear_filled_rows(&mut commands, &mut game, block_query);

        if game.stats.health() > 0.0 {
            spawn_cube(&mut commands, &mut game);
        }
    }
}

fn clear_filled_rows(
    commands: &mut Commands,
    game: &mut Game,
    block_query: Query<(Entity, &GlobalTransform)>,
) {
    let mut blocks_per_row: Vec<Vec<Entity>> = (0..game.n_rows).map(|_| vec![]).collect();

    let floor_y = game.floor_y();

    for (block_entity, position) in block_query.iter() {
        let floor_distance = position.translation().y - floor_y;

        // The center of a block on the floor is 0.5 above the floor, so .floor() the number ;)
        let row = floor_distance.floor() as i32;

        if row >= 0 && row < game.n_rows as i32 {
            blocks_per_row[row as usize].push(block_entity);
        }
    }

    for row_blocks in blocks_per_row {
        if row_blocks.len() == game.n_lanes as usize {
            game.stats.cleared_blocks += game.n_lanes as i32;

            for block_entity in row_blocks {
                commands.entity(block_entity).despawn_recursive();
            }
        }
    }
}
