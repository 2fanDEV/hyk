use std::ops::Deref;

use anyhow::Result;
use bytemuck::{bytes_of, cast_slice, NoUninit};
use wgpu::{util::{BufferInitDescriptor, DeviceExt}, Buffer, BufferDescriptor, BufferUsages, Device};

pub trait BufferElements {}

#[allow(non_camel_case_types)]
pub enum ElementType<'a, T> {
    VECTOR(&'a [T]),
    SINGLE_ELEMENT(&'a T)
}

pub struct MeshBuffer<'a, T> {
    pub vertex_buffer: ElementBuffer<'a, T>,
    pub index_buffer: ElementBuffer<'a, u32>
}

impl <'a, T> MeshBuffer<'a, T> {
    pub fn new(vertex_buffer: ElementBuffer<'a, T>, index_buffer: ElementBuffer<'a, u32>) -> Self {
        Self {
            vertex_buffer,
            index_buffer
        }
    }
}


pub struct ElementBuffer<'a, T> {
    buffer: Buffer,
    size: u32,
    elements: ElementType<'a, T>
}

impl <'a, T> Deref for ElementBuffer<'a, T> {
    type Target = Buffer;

    fn deref(&self) -> &Self::Target {
        &self.buffer
    }
}

impl <'a, T> ElementBuffer<'a, T> {
    pub fn new_mapped(device: &Device, label: Option<&str>, usage: BufferUsages, elements: ElementType<'a, T>) -> Result<ElementBuffer<'a, T>>
    where T: NoUninit {
        let elems: &[u8] = match elements {
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
