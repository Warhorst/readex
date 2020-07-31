use crate::matcher::Matcher;
use crate::regex::and::And;
use crate::regex::matcher::Match;
use crate::regex::not::Not;
use crate::regex::or::Or;
use crate::regex::regex_type::RegexType;
use crate::regex::repeater::Repeater;
use crate::repeat::Repeat;

mod regex_type;
mod matcher;
mod repeater;
mod and;
mod or;
mod not;

pub struct Regex<'a> {
    regex_type: Box<dyn RegexType + 'a>,
    next: Option<Box<Regex<'a>>>,
}

impl<'a> Regex<'a> {
    pub fn matcher(matcher: impl Matcher + 'a) -> Self {
        Regex {
            regex_type: Box::new(Match::new(matcher)),
            next: None,
        }
    }

    pub fn repeat(inner: Regex<'a>, repeat: impl Repeat + 'a) -> Self {
        Regex {
            regex_type: Box::new(Repeater::new(inner, repeat)),
            next: None,
        }
    }

    pub fn and(left: Regex<'a>, right: Regex<'a>) -> Self {
        Regex {
            regex_type: Box::new(And::new(left, right)),
            next: None,
        }
    }

    pub fn or(left: Regex<'a>, right: Regex<'a>) -> Self {
        Regex {
            regex_type: Box::new(Or::new(left, right)),
            next: None,
        }
    }

    pub fn not(inner: Regex<'a>) -> Self {
        Regex {
            regex_type: Box::new(Not::new(inner)),
            next: None,
        }
    }

    pub fn matches(&self, string: &str) -> bool {
        self.regex_type.matches_string(string)
    }
}