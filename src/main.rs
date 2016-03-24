extern crate toml;
extern crate hlua;
extern crate cgmath;
pub mod game;
use game::player::Player;
use game::entity::Entity;

fn main() {
    // let c = config::read_from_default();
    let p = Player::new();
    p.update(vec![]);
}
