use crate::string_pointer::StringPointer;

pub trait RegexType {
    fn matches_string(&self, string_pointer: &mut StringPointer) -> bool;
}