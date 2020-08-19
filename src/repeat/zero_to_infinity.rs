use std::fmt::{Display, Formatter};

use crate::repeat::Repeat;

pub struct ZeroToInfinity;

impl Repeat for ZeroToInfinity {
    fn get_minimum(&self) -> Option<usize> {
        None
    }

    fn get_maximum(&self) -> Option<usize> {
        None
    }
}

impl Display for ZeroToInfinity {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ZtI")
    }
}