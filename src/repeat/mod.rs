pub mod times;

/// Trait of everything that expresses how often a specific
/// char sequence should repeat itself, according to a Regex.
pub trait Repeat {
    fn get_minimum(&self) -> Option<usize>;

    fn get_maximum(&self) -> Option<usize>;
}