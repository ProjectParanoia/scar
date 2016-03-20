pub trait Entity {
    fn update(&self);
    fn should_destroy(&self) -> bool;
    fn predestory(&self);
}
