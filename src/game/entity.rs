use game::event::{Events};


pub trait Entity {
    fn update(&self, events: &Vec<Events>);
}
