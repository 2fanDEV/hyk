use anyhow::Result;

use crate::core::{geometry::vertex3d::Vertex3D, renderable::ui::Mesh};

use super::Loader;

pub struct ObjectParser {

}

impl ObjectParser {
    pub fn new() -> Self {
        Self {}
    }
}

impl Loader for ObjectParser {

    fn load(&self, _path: &std::path::Path) -> Result<Vec<Mesh<Vertex3D>>> {
        Ok(vec![])
    }
}
