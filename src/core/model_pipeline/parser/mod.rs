use std::{collections::HashMap, path::Path};

use anyhow::Result;
use gltf_parser::GltfLoader;
use crate::core::{geometry::vertex3d::Vertex3D, ui::Mesh};

mod gltf_parser;
mod object_parser;

pub trait Loader {
    fn load(&self, path: &Path) -> Result<Vec<Mesh<Vertex3D>>>;
}

pub struct MeshLoader {
    loaders: HashMap<String, Box<dyn Loader>>,
}

impl MeshLoader {

    fn new() -> Self {
        let loaders: HashMap<String, Box<dyn Loader>> = HashMap::from([(".glb".to_string(), Box::new(GltfLoader::new()) as Box<dyn Loader>)]);

        Self{
            loaders
        }
    }   
    
    fn select_parser(&self, path: &Path) -> Vec<Mesh<Vertex3D>> {
        let path_str = path.to_str().unwrap();
        let file_extension = path_str.split(".").collect::<Vec<_>>()[1];
        let get_meshes = self.loaders.get(file_extension).unwrap();
        get_meshes.load(path).unwrap()
    }

    pub fn loaders(&self) -> &HashMap<String, Box<dyn Loader>> {
        &self.loaders
    }
}
