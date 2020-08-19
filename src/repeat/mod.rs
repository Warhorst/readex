use std::fmt::Display;

pub mod times;
pub mod zero_to_infinity;

/// Trait of everything that expresses how often a specific
/// char sequence should repeat itself, according to a Regex.
pub trait Repeat: Display {
    fn get_minimum(&self) -> Option<usize>;

    fn get_maximum(&self) -> Option<usize>;
}