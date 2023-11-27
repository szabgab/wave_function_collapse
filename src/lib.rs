#[global_allocator]
static GLOBAL: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

pub mod prelude {
    pub use crate::*;
    pub use grid::*;
    pub use rule::*;
    pub use tile::*;
}

mod grid;
mod rule;
mod tile;
