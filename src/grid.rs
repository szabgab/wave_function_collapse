#[cfg(not(feature = "syncsend"))]
use std::rc::Rc;
#[cfg(feature = "syncsend")]
use std::sync::Arc as Rc;
use std::{marker::PhantomData, ops::Range, slice::Iter};

use rand::{rngs::StdRng, Rng, SeedableRng};

use crate::prelude::*;

// States

/// Type for implementation of `TypeState` pattern
#[derive(Default, Debug, Clone, Copy)]
pub struct NoTiles;
/// Type for implementation of `TypeState` pattern
#[derive(Default, Debug, Clone)]
pub struct Tiles(Vec<Rc<dyn Tile>>);

/// Type for implementation of `TypeState` pattern
#[derive(Default, Debug, Clone, Copy)]
pub struct NoSize;
/// Type for implementation of `TypeState` pattern
#[derive(Default, Debug, Clone, Copy)]
pub struct Size(usize, usize);
impl From<(usize, usize)> for Size {
    fn from(value: (usize, usize)) -> Self {
        Self(value.0, value.1)
    }
}
impl From<Size> for (usize, usize) {
    fn from(value: Size) -> Self {
        (value.0, value.1)
    }
}

/// Type for implementation of `TypeState` pattern
#[derive(Default, Debug, Clone, Copy)]
pub struct Seed(usize);

#[derive(Default, Debug, Clone, Copy)]
pub struct NoSeed;

/// Type for implementation of `TypeState` pattern
#[derive(Default, Debug, Clone, Copy)]
pub struct Sealed;
/// Type for implementation of `TypeState` pattern
#[derive(Default, Debug, Clone, Copy)]
pub struct UnSealed;

/// Builder for `Grid` type utilising `TypeState` pattern
#[derive(Debug, Default, Clone)]
pub struct GridBuilder<T, S1, S2, S3> {
    tiles: T,
    size: S1,
    seed: S2,
    seal_data: PhantomData<S3>,
}
impl GridBuilder<NoTiles, NoSize, NoSeed, UnSealed> {
    /// Only constructor of this type
    pub fn new() -> Self {
        Self::default()
    }
}
impl<T, S1, S2> GridBuilder<T, S1, S2, UnSealed>
where
    T: Default,
    S1: Default,
    S2: Default,
{
    /**
    Function for changing size of grid built with this type
    ```rust
    use wave_func_collapse::prelude::GridBuilder;
    let grid_builder = GridBuilder::new().with_size((10, 10));
    ```
    */
    pub fn with_size(self, size: (usize, usize)) -> GridBuilder<T, Size, S2, UnSealed> {
        GridBuilder {
            size: size.into(),
            tiles: self.tiles,
            seed: self.seed,
            seal_data: PhantomData,
        }
    }
    /**
    Function for changing tiles of grid built with this type
    ```rust
    use wave_func_collapse::prelude::*;

    #[derive(Debug)]
    pub struct Tile1;
    #[derive(Debug)]
    pub struct Tile2;
    create_tile_unit!(Tile1, Tile1, Tile2;);
    create_tile_unit!(Tile2, Tile2, Tile1;);

    let grid_builder = GridBuilder::new().with_tiles(create_tiles_expr!(Tile1, Tile2,));
    ```
    */
    pub fn with_tiles(self, tiles: Vec<Rc<dyn Tile>>) -> GridBuilder<Tiles, S1, S2, UnSealed> {
        GridBuilder {
            tiles: Tiles(tiles),
            size: self.size,
            seed: self.seed,
            seal_data: PhantomData,
        }
    }
    pub fn with_seed(self, seed: usize) -> GridBuilder<T, S1, Seed, UnSealed> {
        GridBuilder {
            tiles: self.tiles,
            size: self.size,
            seed: Seed(seed),
            seal_data: PhantomData,
        }
    }
}
impl GridBuilder<Tiles, Size, Seed, UnSealed> {
    /**
    Function for sealing your current configuration of `GridBuilder`
    ```rust
    use wave_func_collapse::prelude::*;

    #[derive(Debug)]
    pub struct Tile1;
    #[derive(Debug)]
    pub struct Tile2;
    create_tile_unit!(Tile1, Tile1, Tile2;);
    create_tile_unit!(Tile2, Tile2, Tile1;);

    let grid_builder = GridBuilder::new().with_tiles(create_tiles_expr!(Tile1, Tile2,)).with_size((10, 10)).with_seed(1).seal();
    ```
    */
    pub fn seal(self) -> GridBuilder<Tiles, Size, Seed, Sealed> {
        GridBuilder {
            tiles: self.tiles,
            size: self.size,
            seed: self.seed,
            seal_data: PhantomData,
        }
    }
}

impl GridBuilder<Tiles, Size, Seed, Sealed> {
    /**
    Function for building grid out of `GridBuilder`
    ```rust
    use wave_func_collapse::prelude::*;

    #[derive(Debug)]
    pub struct Tile1;
    #[derive(Debug)]
    pub struct Tile2;
    create_tile_unit!(Tile1, Tile1, Tile2;);
    create_tile_unit!(Tile2, Tile2, Tile1;);

    let grid_builder = GridBuilder::new().with_tiles(create_tiles_expr!(Tile1, Tile2,)).with_size((10, 10)).with_seed(1).seal();
    let grid = grid_builder.build();
    ```
    */
    pub fn build(&self) -> Grid<NotGenerated> {
        Grid {
            tiles: self.tiles.0.clone(),
            current_grid: NotGenerated(Vec::new()),
            size: self.size.into(),
            rng: rand::prelude::StdRng::seed_from_u64(self.seed.0 as _),
            rules: Vec::new(),
        }
    }
}

// States
/// Type for implementation of `TypeState` pattern
#[derive(Default, Debug, Clone)]
pub struct NotGenerated(Vec<Option<Rc<dyn Tile>>>);
/// Type for implementation of `TypeState` pattern
#[derive(Default, Debug, Clone)]
pub struct Generated(Vec<Rc<dyn Tile>>);

/// Type for storing grid data utilising `TypeState pattern`
#[derive(Clone)]
#[cfg_attr(test, derive(Debug))]
pub struct Grid<G> {
    tiles: Vec<Rc<dyn Tile>>,
    current_grid: G, //Vec<Option<Rc<dyn Tile>>>,
    size: (usize, usize),
    rules: Vec<Rule>,
    rng: StdRng,
}

impl Grid<NotGenerated> {
    /// Function for generating grid
    pub fn gen(mut self) -> Grid<Generated> {
        self.gen_vec();
        self.gen_rules();
        self.start_gen();
        while !self.gen_step() {}
        Grid {
            tiles: self.tiles,
            current_grid: Generated(
                self.current_grid
                    .0
                    .iter()
                    .cloned()
                    .map(|x| x.unwrap())
                    .collect(),
            ),
            size: self.size,
            rng: self.rng,
            rules: self.rules,
        }
    }
    fn gen_step(&mut self) -> bool {
        let mut low_entropy: (usize, usize, Vec<Rule>) = (0, usize::MAX, Vec::new());
        for (id, tile) in self.current_grid.0.iter().enumerate() {
            if tile.is_none() {
                let (entropy, rules) = self.get_entropy(id);
                if entropy < low_entropy.1 {
                    low_entropy = (id, entropy, rules);
                }
            }
        }
        if low_entropy.1 == usize::MAX {
            return true;
        }
        let tile = low_entropy.2[self.gen_rand(0..low_entropy.1)]
            .second
            .clone();
        self.current_grid.0[low_entropy.0] = Some(tile);
        false
    }
    fn get_entropy(&self, id: usize) -> (usize, Vec<Rule>) {
        if id >= self.size.0 * self.size.1 {
            return (self.rules.len(), Vec::new());
        }
        let mut nearby_tiles: [usize; 4] = [usize::MAX, usize::MAX, usize::MAX, usize::MAX];
        if id >= self.size.0 {
            nearby_tiles[0] = id - self.size.0;
        }
        if id > 1 {
            nearby_tiles[1] = id - 1;
        }
        if id + 1 < self.size.0 * self.size.1 {
            nearby_tiles[2] = id + 1;
        }
        if id + self.size.0 < self.size.0 * self.size.1 {
            nearby_tiles[3] = id + self.size.0;
        }

        let mut possible_tiles = Vec::new();
        possible_tiles.clone_from(&self.rules);
        for id in nearby_tiles {
            if id == usize::MAX {
                continue;
            }
            if let Some(tile) = self.get_tile(id) {
                let tile_rules = tile.rules();
                possible_tiles = possible_tiles
                    .iter()
                    .filter(|x| {
                        for rule in tile_rules.iter() {
                            if *x == rule {
                                return true;
                            }
                        }
                        false
                    })
                    .cloned()
                    .collect::<Vec<_>>();
            }
        }
        (possible_tiles.len(), possible_tiles)
    }
    #[inline(always)]
    fn get_tile(&self, id: usize) -> &Option<Rc<dyn Tile>> {
        &self.current_grid.0[id]
    }
    fn gen_vec(&mut self) {
        let size = self.size.0 * self.size.1;
        for _ in 0..size {
            self.current_grid.0.push(None);
        }
    }
    fn gen_rules(&mut self) {
        let mut possible_tiles = Vec::new();
        for tile in self.tiles.iter() {
            possible_tiles.push(tile.rules());
        }
        let possible_tiles = possible_tiles.iter().flatten().cloned().collect::<Vec<_>>();
        let mut buf = Vec::new();
        for rule in possible_tiles.iter() {
            let mut good = true;
            for r in buf.iter() {
                if rule == r {
                    good = false;
                }
            }
            if good {
                buf.push(rule.clone());
            }
        }
        self.rules = buf;
    }
    fn start_gen(&mut self) {
        let rand = self.gen_rand(0..self.tiles.len());
        let tile = self.tiles[rand].clone();
        let tile_index = self.gen_rand(0..(self.size.0 * self.size.1));
        self.current_grid.0[tile_index] = Some(tile);
    }
    fn gen_rand(&mut self, range: Range<usize>) -> usize {
        self.rng.gen_range(range)
    }
}
impl Grid<Generated> {
    /// Function for iterating over grid
    pub fn iter(&self) -> Iter<'_, Rc<dyn Tile>> {
        self.current_grid.0.iter()
    }
}

mod macros {
    /// Macro for creating tile vector out of types containing `new` constructor
    #[cfg(not(feature = "syncsend"))]
    #[macro_export]
    macro_rules! create_tiles_ty {
        ($($tile:ty,)*) => {
            vec![$(std::rc::Rc::new(<$tile>::new()),)*]
        };
    }
    #[cfg(not(feature = "syncsend"))]
    #[macro_export]
    macro_rules! create_tiles_expr {
        ($($tile:expr,)*) => {
            vec![$(std::rc::Rc::new($tile),)*]
        };
    }
    #[cfg(feature = "syncsend")]
    #[macro_export]
    macro_rules! create_tiles_ty {
        ($($tile:ty,)*) => {
            vec![$(std::sync::Arc::new(<$tile>::new()),)*]
        };
    }
    /// Macro for creating tile vector out of expressions
    #[cfg(feature = "syncsend")]
    #[macro_export]
    macro_rules! create_tiles_expr {
        ($($tile:expr,)*) => {
            vec![$(std::sync::Arc::new($tile),)*]
        };
    }
    pub use {create_tiles_expr, create_tiles_ty};
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use std::{marker::PhantomData, rc::Rc};

    use crate::create_tile;

    #[derive(Debug, Clone)]
    pub struct Water;
    #[derive(Debug, Clone)]
    pub struct Beach;
    #[derive(Debug, Clone)]
    pub struct Land;
    #[derive(Debug, Clone)]
    pub struct Mountains;

    #[derive(Debug, Clone)]
    pub struct SimpleTile<T> {
        _data: PhantomData<T>,
    }

    impl<T> SimpleTile<T> {
        pub fn new() -> Self {
            Self {
                _data: Default::default(),
            }
        }
    }

    create_tile!(SimpleTile<Water>, SimpleTile::<Beach>::new(););
    create_tile!(SimpleTile<Beach>, SimpleTile::<Water>::new() SimpleTile::<Land>::new(););
    create_tile!(SimpleTile<Land>, SimpleTile::<Beach>::new() SimpleTile::<Mountains>::new(););
    create_tile!(SimpleTile<Mountains>, SimpleTile::<Land>::new(););

    #[test]
    fn gen_map() {
        let size = (100, 100);
        let tiles: Vec<Rc<dyn Tile>> = create_tiles_ty!(
            SimpleTile<Water>,
            SimpleTile<Beach>,
            SimpleTile<Land>,
            SimpleTile<Mountains>,
        );
        let grid_builder = GridBuilder::new()
            .with_size(size)
            .with_tiles(tiles)
            .with_seed(213)
            .seal();
        let grid = grid_builder.build().gen();

        for (id, tile) in grid.iter().enumerate() {
            if id % size.0 == 0 {
                println!();
            }
            if let Some(_) = tile.as_any().downcast_ref::<SimpleTile<Land>>() {
                print!("L");
            } else if let Some(_) = tile.as_any().downcast_ref::<SimpleTile<Water>>() {
                print!("W");
            } else if let Some(_) = tile.as_any().downcast_ref::<SimpleTile<Beach>>() {
                print!("B");
            } else if let Some(_) = tile.as_any().downcast_ref::<SimpleTile<Mountains>>() {
                print!("M");
            }
        }
    }
}
