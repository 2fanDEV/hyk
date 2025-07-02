use std::sync::Arc;

use anyhow::{anyhow, Result};
use wgpu::IndexFormat;
use winit::window::{self, Window};

use crate::core::Core;

const MAX_FRAMES: u32 = 2;

pub struct Renderer {
    pub core: Core,
    pub render_flag: bool,
    frame_index: u32
}

impl Renderer {
    
    pub fn new(window: Arc<Window>) -> Result<Self>{
        let core = Core::init(window)?;
        
        Ok(Self {
            core,
            render_flag: false,
            frame_index: 0
        })
    }

    pub fn render(&mut self) {
        self.draw();
        self.frame_index = (self.frame_index + 1) % MAX_FRAMES;
    }

    pub fn update(&mut self) {
            
    }
    
    pub fn draw(&mut self) -> Result<()> {
        let egui_render_pass = self.core.egui_pass("Main Render Pass").unwrap();
        Ok(())
    }
}
