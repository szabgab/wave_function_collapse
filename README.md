
# Description

`wave_function_collapse` is a rust crate for generating random grids (e.g. games).

# Usage


```rust
// First create tiles like so
use wave_func_collapse::prelude::*;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Water;
#[derive(Debug, Clone)]
pub struct Beach;
#[derive(Debug, Clone)]
pub struct Land;

// And define rules for them using `create_tile!` macro

create_tile_unit!(Water, Water, Beach;);
create_tile_unit!(Beach, Beach, Water Land;);
create_tile_unit!(Land, Land, Beach;);

// Now create `GridBuilder` for defining your grids

let size = (25, 25);
let tiles: Vec<Rc<dyn Tile>> = create_tiles_expr!(
  Water,
  Beach,
  Land,
);
let seed = 123;
let grid_builder = GridBuilder::new().with_size(size).with_tiles(tiles).with_seed(seed).seal();

// One thing to note is that you need to specify type for variable that holds your tiles.
// Create simple grid

let grid = grid_builder.build().gen();

// This will create and generate grid that we want. For debugging purposes lets print want we get.

for (id, tile) in grid.iter().enumerate() {
  if id % size.0 == 0 {
    println!();
  }
  if tile.as_any().downcast_ref::<Water>().is_some() {
    print!("W");
  } else if tile.as_any().downcast_ref::<Beach>().is_some() {
    print!("B");
  } else if tile.as_any().downcast_ref::<Land>().is_some() {
    print!("L");
  }
}
```

For more information on how thins work check out [docs](https://docs.rs/wave_func_collapse/latest/wave_func_collapse/)!
