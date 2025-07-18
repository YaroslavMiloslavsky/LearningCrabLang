#![warn(clippy::all, clippy::pedantic)]
#![allow(unused)]

mod cache;
mod policy;

use crate::policy::eviction::FifoPolicy;
use crate::cache::Cache;

fn main() {
    let mut cache: Cache<String, String> = Cache::new(256, Box::new(FifoPolicy::<String>::new()));
    cache.insert(String::from("KeyA"), String::from("ValueA"));
    println!("{:?}", cache.get_data());
}
