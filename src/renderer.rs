use anyhow::Result;
use winit::window::{self, Window};

use crate::core::Core;

const MAX_FRAMES: u32 = 2;

pub struct Renderer {
    core: Core,
    pub render_flag: bool,
    frame_index: u32
}

impl Renderer {
    
    pub fn new(window: Window) -> Result<Self>{
        let core = Core::init(window)?;
        
        Ok(Self {
            core,
            render_flag: false,
            frame_index: 0
        })
    }

    pub fn draw(&mut self) {
        self.drawing();
        self.frame_index = (self.frame_index + 1) % MAX_FRAMES;
    }
    
    pub fn drawing(&mut self) {

    }
}
