use crate::regex::Regex;
use crate::regex::regex_type::RegexType;
use crate::repeat::Repeat;
use crate::string_pointer::StringPointer;

pub struct Repeater<'a, R: Repeat> {
    inner: Regex<'a>,
    repeat: R,
}

impl<'a, R: Repeat> RegexType for Repeater<'a, R> {
    fn matches_string(&self, string_pointer: &mut StringPointer) -> bool {
        false
    }
}

impl<'a, R: Repeat> Repeater<'a, R> {
    pub fn new(inner: Regex<'a>, repeat: R) -> Self {
        Repeater {
            inner,
            repeat,
        }
    }
}