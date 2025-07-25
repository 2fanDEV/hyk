use std::ops::Deref;

use anyhow::{anyhow, Result};
use bytemuck::{bytes_of, cast_slice, NoUninit, Pod};
use tokio::sync::oneshot;
use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    Buffer, BufferAsyncError, BufferUsages, Device, MapMode,
};

use super::renderable::ui::Scissor;


pub trait BufferElements {}

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum ElementType<T> {
    VECTOR(Vec<T>),
    SINGLE_ELEMENT(T),
}

#[derive(Debug)]
pub struct MeshBuffer<T> {
    pub vertex_buffer: ElementBuffer<T>,
    pub index_buffer: ElementBuffer<u32>,
}

impl<T> MeshBuffer<T> {
    pub fn new(vertex_buffer: ElementBuffer<T>, index_buffer: ElementBuffer<u32>) -> Self {
        Self {
            vertex_buffer,
            index_buffer,
        }
    }
}

#[derive(Debug)]
pub struct ElementBuffer<T> {
    buffer: Buffer,
    pub size: u32,
    pub scissor: Option<Scissor>,
    pub elements: ElementType<T>,
}

impl<T> Deref for ElementBuffer<T> {
    type Target = Buffer;

    fn deref(&self) -> &Self::Target {
        &self.buffer
    }
}

impl<T> ElementBuffer<T> {
    pub fn new_mapped(
        device: &Device,
        label: Option<&str>,
        usage: BufferUsages,
        scissor: Option<Scissor>,
        elements: ElementType<T>,
    ) -> Result<ElementBuffer<T>>
    where
        T: Pod,
    {
        let elems: &[u8] = match &elements {
            ElementType::VECTOR(items) => cast_slice(items),
            ElementType::SINGLE_ELEMENT(item) => bytes_of(item),
        };
        let descriptor = BufferInitDescriptor {
            label,
            contents: elems,
            usage,
        };
        let buffer = device.create_buffer_init(&descriptor);
        let size = buffer.size() as u32;
        Ok(Self {
            buffer,
            size,
            elements,
            scissor,
        })
    }

    pub async fn update_buffer(&mut self, data: impl FnOnce() -> Vec<u8>) -> Result<()> {
        let buffer_for_callback = self.buffer.clone();
        let called_data = data();

        let (tx, rx) = oneshot::channel::<Result<()>>();

        let callback = move |result: Result<(), BufferAsyncError>| match result {
            Ok(()) => {
                let mut mapped_range = buffer_for_callback.get_mapped_range_mut(..);
                mapped_range.copy_from_slice(&called_data);
                buffer_for_callback.unmap();
                let _ = tx.send(Ok(()));
            }
            Err(_) => todo!(),
        };
        self.buffer.map_async(MapMode::Write, .., callback);
        let _ = rx.await.map_err(|e| anyhow!("Failed to receive result from WGPU callback: {:?}", e));
        Ok(())
    }
}
