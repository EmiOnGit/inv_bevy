pub trait Item {
    fn get_thumbnail(&self) -> Option<&Path>;
}
