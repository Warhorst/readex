use crate::string_pointer::StringPointer;

pub mod string;
pub mod any;

pub trait Matcher {
    fn matches(&self, string: String) -> bool;

    fn checked_string_length(&self) -> usize;
}