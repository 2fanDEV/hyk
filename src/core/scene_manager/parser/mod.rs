use std::{collections::HashMap, fmt::Debug, path::Path};

use crate::core::{geometry::vertex3d::Vertex3D, renderable::ui::Mesh};
use anyhow::{anyhow, Result};
use gltf_parser::GltfLoader;

mod gltf_parser;
mod object_parser;

pub trait Loader: Debug {
    fn load(&self, path: &Path) -> Result<Vec<Mesh<Vertex3D>>>;
}

#[derive(Debug)]
pub struct MeshLoader {
    loader_registry: HashMap<String, Box<dyn Loader>>,
}

impl MeshLoader {
    pub fn new() -> Self {
        let loaders: HashMap<String, Box<dyn Loader>> = HashMap::from([(
            ".glb".to_string(),
            Box::new(GltfLoader::new()) as Box<dyn Loader>,
        )]);

        Self {
            loader_registry: loaders,
        }
    }

    pub fn load_mesh(&self, path: &Path) -> Vec<Mesh<Vertex3D>> {
        let loader = match self.select_loader(path) {
            Ok(loaded_loader) => loaded_loader,
            Err(_) => return vec![],
        };
        match loader.load(path) {
            Ok(_) => todo!(),
            Err(_) => todo!(),
        }
    }

    fn select_loader(&self, path: &Path) -> Result<&Box<dyn Loader>> {
        let path_str = path.to_str().unwrap();
        let file_extension = path_str.split(".").collect::<Vec<_>>()[1];
        let loader = self
            .loader_registry
            .get(file_extension)
            .ok_or(anyhow!("Failed to convert loader loading to result!"));
        loader
    }

    pub fn loaders(&self) -> &HashMap<String, Box<dyn Loader>> {
        &self.loader_registry
    }
}
