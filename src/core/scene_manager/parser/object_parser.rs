use anyhow::Result;

use crate::core::{self, geometry::{self, vertex3d::{self, Vertex3D}}, ui::{self, Mesh}};

use super::Loader;

pub struct ObjectParser {

}

impl ObjectParser {
    pub fn new() -> Self {
        Self {}
    }
}

impl Loader for ObjectParser {

    fn load(&self, path: &std::path::Path) -> Result<Vec<Mesh<Vertex3D>>> {
        Ok(vec![])
    }
}
