use bevy::prelude::*;
use bevy::utils::HashMap;

// TODO: Update only when "save" is pressed. 
pub fn save_scene(
    gizmos: Query<&mut Transform, With<bevy_transform_gizmo::GizmoTransformable>>
) {
    for transform in gizmos.iter(){
        println!("transform: {}", transform.translation);
    }
}

// Currently assuming you have all gltfs in assets, just require path.
// value = HashMap<path, transform>
// will include serialized JSON gltf

pub fn transform_to_string(transform: Transform) -> String {
    let mut s: String = String::new();
    let translation = transform.translation.to_array();
    for i in translation.iter() {
        let str = format!("{} ", i);
        s += &str;
    }
    let rotation = transform.rotation.to_array();
    for i in rotation.iter() {
        let str = format!("{} ", i);
        s += &str;
    }
    let scale = transform.scale.to_array();
    for i in scale.iter() {
        let str = format!("{} ", i);
        s += &str;
    }
    s
}

pub fn gltf_to_json () {

}