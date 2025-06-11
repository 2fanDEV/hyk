use anyhow::Result;
use winit::window::{self, Window};

use crate::core::Core;

pub struct Renderer {
    core: Core   
}

impl Renderer {
    
    pub fn new(window: Window) -> Result<Self>{
        let core = Core::init(window)?;
        
        Ok(Self {
            core
        })
    }

}
