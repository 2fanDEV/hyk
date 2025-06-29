use anyhow::Result;
use bytemuck::{bytes_of, cast_slice, NoUninit};
use wgpu::{util::{BufferInitDescriptor, DeviceExt}, Buffer, BufferDescriptor, BufferUsages, Device};

pub trait BufferElements {}

#[allow(non_camel_case_types)]
pub enum ElementType<'a, T> {
    VECTOR(&'a [T]),
    SINGLE_ELEMENT(&'a T)
}


pub struct ElementBuffer<'a, T> {
    buffer: Buffer,
    size: u32,
    elements: &'a [T]
}

impl <'a, T> ElementBuffer<'a, T> {
    fn new_mapped(device: &Device, label: Option<&str>, usage: BufferUsages, elements: ElementType<T>) -> Result<Buffer>
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
        Ok(device.create_buffer_init(&descriptor))
    }
}
