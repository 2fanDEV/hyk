use glm::{Matrix4, Vector4};

pub trait PushConstantType {
    fn as_raw(&self) -> Vec<u8>;

    fn size_in_bytes(&self) -> usize {
        std::mem::size_of_val(self)
    }
}

pub struct EguiPushConstant {
    pub screen_to_clip: glm::Matrix4<f32>,
}

impl EguiPushConstant {
    pub fn new(left: f32, right: f32, top: f32, bottom: f32, near: f32, far: f32) -> Self {
        // Create an orthographic projection for egui
        // For Vulkan with (0,0) at top-left mapping to NDC
        let matrix = Matrix4::new(
            Vector4::new(2.0/(right-left), 0.0, 0.0, -((right+left)/(right-left))),
            Vector4::new(0.0, 2.0/(bottom-top), 0.0, -((bottom+top)/(bottom-top))),  // Note: top/bottom order to flip Y
            Vector4::new(0.0, 0.0, 1.0/(far-near), -(near/(far-near))),
            Vector4::new(0.0, 0.0, 0.0, 1.0));
        Self {
            screen_to_clip: matrix
        }
    }
}

impl PushConstantType for EguiPushConstant {
    fn as_raw(&self) -> Vec<u8> {
        let arr: [[f32; 4]; 4] = self.screen_to_clip.as_array().map(|v| *v.as_array());
        bytemuck::bytes_of(&arr).to_vec()
    }
}

#[cfg(test)]
mod tests {}
