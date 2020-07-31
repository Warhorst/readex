use crate::regex::Regex;
use crate::regex::regex_type::RegexType;
use crate::repeat::Repeat;

pub struct Repeater<'a> {
    inner: Regex<'a>,
    repeat: Box<dyn Repeat + 'a>,
}

impl<'a> RegexType for Repeater<'a> {
    fn matches_string(&self, string: &str) -> bool {
        false
    }
}

impl<'a> Repeater<'a> {
    pub fn new(inner: Regex<'a>, repeat: impl Repeat + 'a) -> Self {
        Repeater {
            inner,
            repeat: Box::new(repeat),
        }
    }
}