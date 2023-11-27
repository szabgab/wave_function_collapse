use std::{any::Any, rc::Rc};

use crate::rule::Rule;

pub trait Tile: std::fmt::Debug + std::any::Any {
    fn type_str(&self) -> &str {
        std::any::type_name::<Self>()
    }
    fn as_any(&self) -> &dyn Any;
    fn rules(&self) -> Vec<Rule> {
        vec![]
    }
    fn rules_static() -> Vec<Rule>
    where
        Self: Sized,
    {
        vec![]
    }
}
pub trait Pack: Sized + Tile + 'static {
    fn pack(self) -> Rc<dyn Tile> {
        unsafe { Rc::<dyn Tile>::from_raw(Rc::into_raw(Rc::new(self))) }
    }
}

impl<T> Pack for T where T: Tile + 'static {}

mod macros {
    #[macro_export]
    macro_rules! create_tile {
        ($tile_type:ty, $($rule: expr)*;) => {
            impl Tile for $tile_type {
                fn as_any(&self) -> &dyn Any {
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
    pub use create_tile;
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
