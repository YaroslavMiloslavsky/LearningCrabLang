#![warn(clippy::all, clippy::pedantic)]

pub mod arena;
pub mod player;

pub use arena::{Arena, ArenaErr, Event};
pub use player::Player;
