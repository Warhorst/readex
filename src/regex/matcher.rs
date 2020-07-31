use crate::matcher::Matcher;
use crate::regex::regex_type::RegexType;

pub struct Match<'a> {
    matcher: Box<dyn Matcher + 'a>
}

impl<'a> RegexType for Match<'a> {
    fn matches_string(&self, string: &str) -> bool {
        false
    }
}

impl<'a> Match<'a> {
    pub fn new(matcher: impl Matcher + 'a) -> Self {
        Match {
            matcher: Box::new(matcher)
        }
    }
}