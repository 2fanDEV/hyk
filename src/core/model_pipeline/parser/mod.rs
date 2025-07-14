use std::{collections::HashMap, path::Path};

use gltf_parser::parse_gltf;

use crate::core::{geometry::vertex3d::Vertex3D, ui::Meshes};

mod gltf_parser;

fn select_parser(path: &Path) -> Vec<Meshes<Vertex3D>> {
    let path_str = path.to_str().unwrap();
    let file_extension = path_str.split(".").collect::<Vec<_>>()[1];
    let map = HashMap::from([(".glb", || parse_gltf(path))]);
    let get_meshes = map.get(file_extension).unwrap();
    get_meshes().unwrap()
}
