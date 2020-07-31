use crate::matcher::Matcher;
use crate::regex::and::And;
use crate::regex::matcher::Match;
use crate::regex::not::Not;
use crate::regex::or::Or;
use crate::regex::regex_type::RegexType;
use crate::regex::repeater::Repeater;
use crate::repeat::Repeat;
use crate::string_pointer::StringPointer;

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

    /// This method is used to chain multiple regexes together.
    /// The given next regex is always appended to the last
    /// element in the chain.
    ///
    /// Examples:
    /// ```
    /// use crate::readex::regex::Regex;
    /// use crate::readex::matcher::string::Str;
    /// use crate::readex::matcher::any::Any;
    ///
    /// let regex_one = Regex::matcher(Str::new("foo"))
    ///     .followed_by(Regex::matcher(Any))
    ///     .followed_by(Regex::matcher(Str::new("bar")))
    ///     .followed_by(Regex::matcher(Any))
    ///     .followed_by(Regex::matcher(Str::new("baz")));
    ///
    /// let regex_two = Regex::matcher(Str::new("foo"))
    ///     .followed_by(Regex::matcher(Any)
    ///         .followed_by(Regex::matcher(Str::new("bar"))
    ///             .followed_by(Regex::matcher(Any)
    ///                 .followed_by(Regex::matcher(Str::new("baz"))))));
    ///
    /// assert!(regex_one.matches("foo bar baz"));
    /// assert!(regex_two.matches("foo bar baz"));
    /// ```
    pub fn followed_by(mut self, next: Regex<'a>) -> Self {
        match self.next {
            Some(next_regex) => Regex {
                regex_type: self.regex_type,
                next: Some(Box::new(next_regex.followed_by(next))),
            },
            None => {
                self.next = Some(Box::new(next));
                self
            }
        }
    }

    pub fn matches(&self, string: &str) -> bool {
        let mut string_pointer = StringPointer::from(string);
        self.matches_string(&mut string_pointer)
    }

    fn matches_string(&self, string_pointer: &mut StringPointer) -> bool {
        let matches = self.regex_type.matches_string(string_pointer);
        match (matches, &self.next) {
            (true, Some(next_regex)) => next_regex.matches_string(string_pointer),
            (true, None) => true,
            _ => false
        }
    }
}