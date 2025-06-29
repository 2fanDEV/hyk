pub trait PushConstantType {
    fn as_raw(&self) -> &[u8];

    fn size_in_bytes(&self) -> usize {
        std::mem::size_of_val(self)
    }
}

#[cfg(test)]
mod tests {}
