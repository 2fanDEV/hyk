pub enum BufferInput<T> {
    Single(T),
    Multiple(Vec<T>)
}
