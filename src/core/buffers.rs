use std::ops::{Deref, RangeBounds};

use anyhow::Result;
use bytemuck::{bytes_of, cast_slice, NoUninit};
use wgpu::{
    util::{BufferInitDescriptor, DeviceExt}, Buffer, BufferAddress, BufferAsyncError, BufferDescriptor, BufferUsages, Device, MapMode
};

use super::ui::Scissor;

pub trait BufferElements {}

#[allow(non_camel_case_types)]
pub enum ElementType<T> {
    VECTOR(Vec<T>),
    SINGLE_ELEMENT(T),
}

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
        T: NoUninit
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

    pub fn update_buffer<B: RangeBounds<BufferAddress>>(&mut self, device: &Device, bounds: B, data: impl FnOnce() -> Vec<u8>)-> Result<()> {
        let callback = move |result: Result<(), BufferAsyncError>| {
                let get_mapped_range_mut = self.slice(..).get_mapped_range_mut().copy_from_slice(&data());    
        };
        self.buffer.map_async(MapMode::Write, .., callback);
        Ok(())
    } 
}
