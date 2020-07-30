use crate::regex::Regex;

pub enum Expression<'a> {
    And(Regex<'a>, Regex<'a>),
    Or(Regex<'a>, Regex<'a>),
    Not(Regex<'a>),
}