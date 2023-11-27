#![doc = include_str!("../README.md")]

#[global_allocator]
static GLOBAL: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

pub mod prelude {
    pub use crate::*;
    pub use grid::*;
    pub use rule::*;
    pub use tile::*;
}

pub mod grid;
pub mod rule;
pub mod tile;
