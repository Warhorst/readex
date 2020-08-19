use std::fmt::{Display, Formatter};

use crate::repeat::Repeat;

/// A Repeat with an exact number of how often a
/// char sequence should repeat.
pub struct Times {
    repeats: usize
}

impl Times {
    pub fn new(repeats: usize) -> Self {
        Times {
            repeats
        }
    }
}

impl Repeat for Times {
    fn get_minimum(&self) -> Option<usize> {
        Some(self.repeats)
    }

    fn get_maximum(&self) -> Option<usize> {
        Some(self.repeats)
    }
}

impl Display for Times {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Times")
    }
}