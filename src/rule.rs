#[cfg(not(feature = "syncsend"))]
use std::rc::Rc;
#[cfg(feature = "syncsend")]
use std::sync::Arc as Rc;

use crate::{prelude::Pack, tile::Tile};

/// Create for storing info about possible adjacent tiles.
#[derive(Debug, Clone)]
pub struct Rule {
    pub(crate) second: Rc<dyn Tile>,
}

impl PartialEq for Rule {
    fn eq(&self, other: &Self) -> bool {
        self.second.type_str() == other.second.type_str()
    }
}
impl Eq for Rule {}

impl Rule {
    /// Contructor function
    pub fn new(second: impl Pack) -> Self {
        Self {
            second: second.pack(),
        }
    }
}
