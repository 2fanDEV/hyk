use nalgebra::Matrix4;

pub trait PushConstantType {
    fn as_raw(&self) -> Vec<u8>;

    fn size_in_bytes(&self) -> usize {
        std::mem::size_of_val(self)
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct EguiPushConstant {
    pub screen_to_clip: Matrix4<f32>,
}

impl EguiPushConstant {
    pub fn new(width: f32, height: f32, pixels_per_point: f32) -> Self {
        let matrix =
            Matrix4::<f32>::new(
            pixels_per_point * 2.0 / width, 0.0, 0.0, -1.0,
            0.0, pixels_per_point * -2.0 / height, 0.0,1.0,
            0.0,0.0,1.0,0.0,
            0.0,0.0, 0.0,1.0,
        );
        Self {
            screen_to_clip: matrix,
        }
    }
}

impl PushConstantType for EguiPushConstant {
    fn as_raw(&self) -> Vec<u8> {
        let data_ptr = &self.screen_to_clip as *const Matrix4<f32>;
        unsafe {
            std::slice::from_raw_parts(data_ptr as *const u8, size_of::<Matrix4<f32>>()).to_vec()
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct ScenePushConstant {}

impl PushConstantType for ScenePushConstant {
    fn as_raw(&self) -> Vec<u8> {
        todo!()
    }
}

#[cfg(test)]
mod tests {}
