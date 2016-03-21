pub trait Entity {
    fn update(&self, &Vec<Event>);
    fn should_destroy(&self) -> bool;
    fn predestory(&self);
}
