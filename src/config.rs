use std::fs::File;
use std::io::prelude::*;
use toml;
use toml::Value;

pub struct Config {
    pub title: String,
    pub winsize: f64,
    pub fullscreen: bool,
    pub main: String, // main file for lua integration
}

pub fn read_from_default() -> Config {
    let mut input = String::new();
    File::open("config.toml").and_then(|mut f| {
        f.read_to_string(&mut input)
    }).unwrap();

    let mut parser = toml::Parser::new(&input);
    let toml = match parser.parse() {
        Some(toml) => toml,
        None => {
            panic!("Parser error: config file is not a valid toml file.");
        }
    };

    return Config {
        title: match toml.get("title").unwrap() {
            &Value::String(ref x) => x.clone(),
            _ => panic!("Config error: title should be a string.")
        },
        winsize: match toml.get("winsize").unwrap() {
            &Value::Float(x) => x,
            _ => panic!("Config error: winsize should be a float.")
        },
        fullscreen: match toml.get("fullscreen").unwrap() {
            &Value::Boolean(x) => x,
            _ => panic!("Config error: fullscreen should be a bool.")
        },
        main: match toml.get("main").unwrap() {
            &Value::String(ref x) => x.clone(),
            _ => panic!("Config error: main should be a string.")
        },
    }
}
