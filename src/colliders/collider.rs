use bevy::prelude::*;
use bevy::render::render_asset::RenderAsset;
use bevy_rapier3d::prelude::ComputedColliderShape::ConvexDecomposition;
use bevy_rapier3d::prelude::*;

#[derive(Debug, Default)]
pub struct ColliderBuilderPlugin;

impl Plugin for ColliderBuilderPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_system(mesh_collider_create);
        app.add_system_to_stage(CoreStage::PostUpdate, mesh_collider_transform);
        app.insert_resource(Run((false, usize::MIN, usize::MIN)));
    }
}

#[derive(Component)]
pub struct BetterParent((Entity, Handle<Scene>));
impl BetterParent {
    pub fn new(e: Entity, handle: Handle<Scene>) -> BetterParent {
        BetterParent((e, handle))
    }
}

#[derive(Component)]
pub struct AddCollider((bool, Handle<Scene>));
impl AddCollider {
    pub fn new(bool: bool, handle: Handle<Scene>) -> AddCollider {
        AddCollider((bool, handle))
    }
}

#[derive(Component)]
pub struct ConsumedCollider(Handle<Scene>);
impl ConsumedCollider {
    pub fn new(handle: Handle<Scene>) -> ConsumedCollider {
        ConsumedCollider(handle)
    }
}

// Whether first loop is done, Total colliders to create, colliders finished
pub struct Run((bool, usize, usize));

pub fn mesh_collider_transform(
    mut commands: Commands,
    q_child: Query<(Entity, &BetterParent)>,
    mut ass_world: ResMut<Assets<Scene>>,
    mut run: ResMut<Run>,
    parent_mesh_transform: Query<(Entity, &Handle<Mesh>)>,
    scene_select: Query<(Entity, &ConsumedCollider)>,
) {
    // If mesh has been rendered and one loop has passed, and this function has not been called
    if run.0 .0 & (run.0 .1 > 0) & (run.0 .2 != run.0 .1) {
        while run.0 .1 > 0 {
            for (ent, cder) in scene_select.iter() {
                let collider_info = cder.0.clone();
                match ass_world.get_mut(&collider_info) {
                    Some(world) => {
                        let mut query_one = world
                            .world
                            .query::<(Entity, &Handle<Mesh>, &GlobalTransform)>();

                        for (c_entity, bp) in q_child.iter() {
                            if bp.0 .1 == collider_info {
                                let prnt = query_one.get(&mut world.world, bp.0 .0);
                                for (p_entity, p_handle) in parent_mesh_transform.iter() {
                                    if p_handle == prnt.unwrap().1 {
                                        /*commands.entity(e).insert_bundle(TransformBundle::from(
                                            Transform::from(*c),
                                        ));*/

                                        commands.entity(p_entity).push_children(&[c_entity]);

                                        break;
                                    }
                                }
                            }
                        }
                        // println!("{:?}", run.0 .1);
                        run.0 .1 = run.0 .1 - 1;
                        if run.0 .1 == 0 {
                            run.0 .2 = 0;
                        }
                        commands.entity(ent).remove::<ConsumedCollider>();
                    }

                    None => (),
                }
            }
        }
    } else if run.0 .0 & (run.0 .1 > 0) & (run.0 .2 == run.0 .1) {
        run.0 .2 += 1;
    }
}

pub fn mesh_collider_create(
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    mut ass_world: ResMut<Assets<Scene>>,
    mut run: ResMut<Run>,
    objects: Query<(Entity, &AddCollider)>,
) {
    // watches for changes
    if objects.iter().len() > 0 {
        run.0 .1 = objects.iter().len();
        run.0 .0 = false;
    }
    if !run.0 .0 {
        run.0 .1 = objects.iter().len();
        for (e, tagged_entities) in objects.iter() {
            let collider_info = tagged_entities.0.clone();
            match ass_world.get_mut(&collider_info.1) {
                Some(world) => {
                    let mut query_one = world.world.query::<(Entity, &Handle<Mesh>)>();
                    for (entity, mesh) in query_one.iter_mut(&mut world.world) {
                        let parent = commands.entity(entity).id();
                        if collider_info.0 {
                            let collider = Collider::from_bevy_mesh(
                                &meshes.get(&mesh).unwrap().extract_asset(),
                                &ComputedColliderShape::TriMesh,
                            )
                            .unwrap();
                            commands
                                .spawn()
                                .insert(collider)
                                .insert(Friction::new(1000.0))
                                .insert(BetterParent::new(parent, collider_info.1.clone()))
                                .insert_bundle(TransformBundle::default());
                        } else {
                            let collider = Collider::from_bevy_mesh(
                                &meshes.get(&mesh).unwrap().extract_asset(),
                                &ConvexDecomposition(VHACDParameters::default()),
                            )
                            .unwrap();
                            commands
                                .spawn()
                                .insert(collider)
                                .insert(Friction::new(1000.0))
                                .insert(BetterParent::new(parent, collider_info.1.clone()))
                                .insert_bundle(TransformBundle::default());
                        }
                    }
                    commands.entity(e).remove::<AddCollider>();
                    commands
                        .entity(e)
                        .insert(ConsumedCollider::new(collider_info.1.clone()));
                    run.0 .2 += 1;
                    if (run.0 .1 == 1) || (run.0 .2 == run.0 .1) {
                        run.0 .0 = true;
                        run.0 .1 = run.0 .2;
                    }
                }

                None => (),
            }
        }
    }
}
