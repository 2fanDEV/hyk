use std::ops::Deref;

use anyhow::Result;
use bytemuck::{bytes_of, cast_slice, NoUninit};
use wgpu::{util::{BufferInitDescriptor, DeviceExt}, Buffer, BufferDescriptor, BufferUsages, Device};

pub trait BufferElements {}

#[allow(non_camel_case_types)]
pub enum ElementType<T> {
    VECTOR(Vec<T>),
    SINGLE_ELEMENT(T)
}

pub struct MeshBuffer<T> {
    pub vertex_buffer: ElementBuffer<T>,
    pub index_buffer: ElementBuffer<u32>
}

impl <T> MeshBuffer<T> {
    pub fn new(vertex_buffer: ElementBuffer<T>, index_buffer: ElementBuffer<u32>) -> Self {
        Self {
            vertex_buffer,
            index_buffer
        }
    }
}


pub struct ElementBuffer<T> {
    buffer: Buffer,
    pub size: u32,
    pub elements: ElementType<T>
}

impl <T> Deref for ElementBuffer<T> {
    type Target = Buffer;

    fn deref(&self) -> &Self::Target {
        &self.buffer
    }
}

impl <T> ElementBuffer<T> {
    pub fn new_mapped(device: &Device, label: Option<&str>, usage: BufferUsages, elements: ElementType<T>) -> Result<ElementBuffer<T>>
    where T: NoUninit {
        let elems: &[u8] = match &elements {
            ElementType::VECTOR(items) => cast_slice(items),
            ElementType::SINGLE_ELEMENT(item) => bytes_of(item) ,
        };
        let descriptor = BufferInitDescriptor {
            label,
            contents: elems,
            usage
        };
        let buffer = device.create_buffer_init(&descriptor);
        let size = buffer.size() as u32;
        Ok(Self {
            buffer,
            size,
            elements
        }
            )
    }
}
