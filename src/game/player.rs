use game::event::{Events};
use game::keys::{Keys};
use game::entity::Entity;

pub struct Player {
    x: f64,
    y: f64,
    vx: f64,
    vy: f64,
}

impl Player {
    fn new() -> Player {
        return Player {x: 0.0, y: 0.0, vx: 0.0, vy: 0.0};
    }
}

impl Entity for Player {
    fn update(&self, events: &Vec<Events>) {
        for e in events.iter() {
            match e {
                Events::Keypress(Keys::Up) => {
                    println!("Up pressed");
                },
            }
        }
    }
}
