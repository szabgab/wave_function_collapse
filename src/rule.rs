use std::rc::Rc;

use crate::{prelude::Pack, tile::Tile};

#[derive(Clone)]
#[cfg_attr(test, derive(Debug))]
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
    pub fn new(second: impl Pack) -> Self {
        Self {
            second: second.pack(),
        }
    }
}
