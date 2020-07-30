use crate::expression::Expression;
use crate::matcher::Matcher;
use crate::repeat::Repeat;

pub struct Regex<'a> {
    next: Option<Box<Regex<'a>>>,
    matcher: Box<dyn Matcher + 'a>,
    repeat: Option<Box<dyn Repeat + 'a>>,
}

impl<'a> Regex<'a> {
    pub fn for_matcher(matcher: impl Matcher + 'a) -> Self {
        Regex {
            next: None,
            matcher: Box::new(matcher),
            repeat: None,
        }
    }

    pub fn matches_string(&self, string: &str) -> bool {
        false
    }
}

enum RegexType<'a> {
    Match(Box<dyn Matcher + 'a>),
    Repeat(Regex<'a>, Box<dyn Repeat + 'a>),
    Expression(Expression<'a>),
}