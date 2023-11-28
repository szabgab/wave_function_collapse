use std::any::Any;
#[cfg(not(feature = "syncsend"))]
use std::rc::Rc;
#[cfg(feature = "syncsend")]
use std::sync::Arc as Rc;

use crate::rule::Rule;

/// Tile trait
#[cfg(not(feature = "syncsend"))]
pub trait Tile: std::fmt::Debug + std::any::Any {
    /**
    Internal method for comparing tiles
    */
    fn type_str(&self) -> &str {
        std::any::type_name::<Self>()
    }
    /**
    Function for downcasting tile into concrete type
    ```rust
    use std::rc::Rc;
    use std::any::Any;
    use wave_func_collapse::prelude::*;

    #[derive(Debug)]
    struct SimpleTile;
    create_tile_unit!(SimpleTile, SimpleTile,;);
    let tile: Rc<dyn Tile> = Rc::new(SimpleTile);

    if let Some(concrete) = tile.as_any().downcast_ref::<SimpleTile>() {
        println!("{concrete:?}");
    }
    ```
    */
    fn as_any(&self) -> &dyn Any;
    /**
    Function for getting rules from a trait object
    ```rust
    use std::rc::Rc;
    use std::any::Any;
    use wave_func_collapse::prelude::*;

    #[derive(Debug)]
    struct SimpleTile1;
    #[derive(Debug)]
    struct SimpleTile2;
    create_tile_unit!(SimpleTile1, SimpleTile1, SimpleTile2;);
    create_tile_unit!(SimpleTile2, SimpleTile2, SimpleTile1;);
    let tile: Rc<dyn Tile> = Rc::new(SimpleTile1);
    println!("{:?}", tile.rules())

    ```
    */
    fn rules(&self) -> Vec<Rule> {
        vec![]
    }
    /**
    Same as `rules` but static
    */
    fn rules_static() -> Vec<Rule>
    where
        Self: Sized,
    {
        vec![]
    }
}

#[cfg(feature = "syncsend")]
pub trait Tile: std::fmt::Debug + std::any::Any + Sync + Send {
    /**
    Internal method for comparing tiles
    */
    fn type_str(&self) -> &str {
        std::any::type_name::<Self>()
    }
    /**
    Function for downcasting tile into concrete type
    ```rust
    use std::rc::Rc;
    use std::any::Any;
    use wave_function_collapse::prelude::*;

    #[derive(Debug)]
    struct SimpleTile;
    create_tile_unit!(SimpleTile, SimpleTile,;);
    let tile: Rc<dyn Tile> = Rc::new(SimpleTile);

    if let Some(concrete) = tile.as_any().downcast_ref::<SimpleTile>() {
        println!("{concrete:?}");
    }
    ```
    */
    fn as_any(&self) -> &dyn Any;
    /**
    Function for getting rules from a trait object
    ```rust
    use std::rc::Rc;
    use std::any::Any;
    use wave_function_collapse::prelude::*;

    #[derive(Debug)]
    struct SimpleTile1;
    #[derive(Debug)]
    struct SimpleTile2;
    create_tile_unit!(SimpleTile1, SimpleTile1, SimpleTile2;);
    create_tile_unit!(SimpleTile2, SimpleTile2, SimpleTile1;);
    let tile: Rc<dyn Tile> = Rc::new(SimpleTile1);
    println!("{:?}", tile.rules())

    ```
    */
    fn rules(&self) -> Vec<Rule> {
        vec![]
    }
    /**
    Same as `rules` but static
    */
    fn rules_static() -> Vec<Rule>
    where
        Self: Sized,
    {
        vec![]
    }
}

/// A helper trait
pub trait Pack: Sized + Tile + 'static {
    fn pack(self) -> Rc<dyn Tile> {
        unsafe { Rc::<dyn Tile>::from_raw(Rc::into_raw(Rc::new(self))) }
    }
}

impl<T> Pack for T where T: Tile + 'static {}

/// Utility macros
mod macros {
    /**
    Macro for creating implementation of `Tile` trait for structs that have `new` constructor
    */
    #[macro_export]
    macro_rules! create_tile {
        ($tile_type:ty, $($rule: expr)*;) => {
            impl Tile for $tile_type {
                fn as_any(&self) -> &dyn std::any::Any {
                    self
                }
                fn rules(&self) -> Vec<Rule> {
                    let mut v = vec![$(Rule::new($rule),)*];
                    v.push(Rule::new(<$tile_type>::new()));
                    v
                }
                fn rules_static() -> Vec<Rule>
                where
                    Self: Sized,
                {
                    let mut v = vec![$(Rule::new($rule),)*];
                    v.push(Rule::new(<$tile_type>::new()));
                    v
                }
            }
        };
    }
    /**
    Macro for creating implementation of `Tile` trait for unit structs
    */
    #[macro_export]
    macro_rules! create_tile_unit {
        ($tile_type:ty, $tile_type_expr:expr, $($rule: expr)*;) => {
            impl Tile for $tile_type {
                fn as_any(&self) -> &dyn std::any::Any {
                    self
                }
                fn rules(&self) -> Vec<Rule> {
                    let mut v = vec![$(Rule::new($rule),)*];
                    v.push(Rule::new($tile_type_expr));
                    v
                }
                fn rules_static() -> Vec<Rule>
                where
                    Self: Sized,
                {
                    let mut v = vec![$(Rule::new($rule),)*];
                    v.push(Rule::new($tile_type_expr));
                    v
                }
            }
        };
    }
    
}

#[cfg(test)]
mod tests {
    use std::marker::PhantomData;

    use crate::create_tile;

    use super::*;

    #[derive(Debug, Clone)]
    pub struct Water;
    #[derive(Debug, Clone)]
    pub struct Beach;
    #[derive(Debug, Clone)]
    pub struct Land;

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
    create_tile!(SimpleTile<Land>, SimpleTile::<Beach>::new(););

    // #[test]
    // pub fn get_rules() {
    //     println!("{:#?}", SimpleTile::<Beach>::rules_static());
    // }
}
