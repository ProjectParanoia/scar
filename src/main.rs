extern crate toml;
extern crate hlua;
pub mod config;
pub mod lua_core;

fn main() {
    let c = config::read_from_default();
    println!("title: {}", c.title)
}
