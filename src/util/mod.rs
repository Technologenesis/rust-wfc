pub trait CloneBox<T> {
    fn clone_box(&self) -> Box<T>;
}