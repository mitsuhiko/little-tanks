#![feature(phase, if_let, macro_rules)]

extern crate serialize;
extern crate gl;
extern crate gfx;
extern crate time;
extern crate device;
extern crate render;
extern crate cgmath;
#[phase(plugin)]
extern crate gfx_macros;
extern crate glfw;
extern crate image;

mod macros;

pub mod errors;
pub mod map;
pub mod engine;
pub mod game;
pub mod texture;
pub mod resources;
pub mod meshutils;


fn main() {
    game::run();
}
