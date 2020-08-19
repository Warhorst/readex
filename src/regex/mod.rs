use std::fmt::{Display, Formatter};
use std::ops::Range;

use crate::matcher::Matcher;
use crate::regex::and::And;
use crate::regex::matcher::Match;
use crate::regex::not::Not;
use crate::regex::or::Or;
use crate::regex::regex_type::RegexType;
use crate::repeat::Repeat;
use crate::string_pointer::{StringPointer, StringPointerError};

mod regex_type;
mod matcher;
mod and;
mod or;
mod not;

pub struct Regex<'a> {
    regex_type: Box<dyn RegexType + 'a>,
    next: Option<Box<Regex<'a>>>,
    repeat: Option<Box<dyn Repeat + 'a>>,
}

impl<'a> Regex<'a> {
    pub fn matcher(matcher: impl Matcher + 'a) -> Self {
        Self::new_regex(Match::new(matcher))
    }

    pub fn and(left: Regex<'a>, right: Regex<'a>) -> Self {
        Self::new_regex(And::new(left, right))
    }

    pub fn or(left: Regex<'a>, right: Regex<'a>) -> Self {
        Self::new_regex(Or::new(left, right))
    }

    pub fn not(inner: Regex<'a>) -> Self {
        Self::new_regex(Not::new(inner))
    }

    fn new_regex(regex_type: impl RegexType + 'a) -> Self {
        Regex {
            regex_type: Box::new(regex_type),
            next: None,
            repeat: None,
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
                repeat: self.repeat,
            },
            None => {
                self.next = Some(Box::new(next));
                self
            }
        }
    }

    pub fn that_repeats(mut self, repeat: impl Repeat + 'a) -> Self {
        self.repeat = Some(Box::new(repeat));
        self
    }

    pub fn matches(&self, string: &str) -> bool {
        let mut string_pointer = StringPointer::from(string);
        let match_result = self.matches_string(&mut string_pointer);

        if let Err(StringPointerError::SizeExceeded) = match_result {
            return false;
        };

        match_result.unwrap()
    }

    fn matches_string(&self, string_pointer: &mut StringPointer) -> Result<bool, StringPointerError> {
        match &self.repeat {
            None => self.matches_string_without_repeat(string_pointer),
            Some(repeat) => self.matches_string_with_repeat(string_pointer, repeat.get_minimum().unwrap_or(0), repeat.get_maximum())
        }
    }

    fn matches_string_without_repeat(&self, string_pointer: &mut StringPointer) -> Result<bool, StringPointerError> {
        let regex_matches_result = self.own_regex_matches(string_pointer);
        if let Ok(true) = regex_matches_result {
            return self.next_regex_matches(string_pointer);
        }
        regex_matches_result
    }

    fn matches_string_with_repeat(&self, string_pointer: &mut StringPointer, min_repeat: usize, max_repeat: Option<usize>) -> Result<bool, StringPointerError> {
        match self.minimum_repeat_matches(string_pointer, min_repeat) {
            Ok(true) => return Ok(true),
            Err(error) => return Err(error),
            _ => ()
        }

        let mut counter = min_repeat;
        while !self.counter_at_max(counter, &max_repeat) {
            let own_regex_matches = self.own_regex_matches(string_pointer);
            string_pointer.set_checkpoint();

            match own_regex_matches {
                Ok(true) => match self.next_regex_matches(string_pointer) {
                    Ok(true) => return Ok(true),
                    Ok(false) => string_pointer.return_to_checkpoint().unwrap(),
                    Err(error) => return Err(error),
                },
                Ok(false) => string_pointer.return_to_checkpoint().unwrap(),
                Err(error) => return Err(error),
            }
        }

        Ok(false)
    }

    /// Returns if this Regex matches the given StringPointer a minimum amount of times, defined
    /// by the set Repeat. Afterwards, the following regexes must match the remaining
    /// string. If this is not true, the StringPointer is reset to its index that was reached after
    /// the repeats.
    fn minimum_repeat_matches(&self, string_pointer: &mut StringPointer, min: usize) -> Result<bool, StringPointerError> {
        for _ in 0..min {
            match self.own_regex_matches(string_pointer) {
                Ok(false) => return Ok(false),
                Err(error) => return Err(error),
                _ => ()
            }
        }

        string_pointer.set_checkpoint();

        match self.next_regex_matches(string_pointer) {
            Ok(true) => Ok(true),
            Err(error) => Err(error),
            _ => {
                string_pointer.return_to_checkpoint().unwrap();
                Ok(false)
            }
        }
    }

    fn counter_at_max(&self, counter: usize, max_repeat: &Option<usize>) -> bool {
        match max_repeat {
            Some(value) => &counter == value,
            None => false
        }
    }

    /// Returns if this Regex, defined by its type, matches the given StringPointer.
    fn own_regex_matches(&self, string_pointer: &mut StringPointer) -> Result<bool, StringPointerError> {
        self.regex_type.matches_string(string_pointer)
    }

    /// Returns if the following Regex of this one matches the given StringPointer.
    /// If no following Regex is set, the StringPointer must have reached its end.
    fn next_regex_matches(&self, string_pointer: &mut StringPointer) -> Result<bool, StringPointerError> {
        match &self.next {
            Some(next_regex) => next_regex.matches_string(string_pointer),
            None => Ok(string_pointer.at_the_end())
        }
    }
}

impl<'a> Display for Regex<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut regex_string = String::new();
        regex_string.push_str(format!("Type: {} \n", self.regex_type).as_str());
        regex_string.push_str(format!("Repeat: {}\n", match &self.repeat {
            Some(repeat) => repeat.to_string(),
            None => String::from("None")
        }).as_str());
        regex_string.push_str(format!("Next Regex: \n{}", match &self.next {
            Some(next_regex) => next_regex.to_string(),
            None => String::from("None")
        }).as_str());

        write!(f, "{}", regex_string)
    }
}