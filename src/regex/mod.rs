use crate::matcher::Matcher;
use crate::regex::and::And;
use crate::regex::matcher::Match;
use crate::regex::not::Not;
use crate::regex::or::Or;
use crate::regex::regex_type::RegexType;
use crate::repeat::Repeat;
use crate::string_pointer::StringPointer;

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
        self.matches_string(&mut string_pointer)
    }

    fn matches_string(&self, string_pointer: &mut StringPointer) -> bool {
        match &self.repeat {
            Some(repeat) => {
                self.matches_string_with_repeat(string_pointer, repeat.get_minimum(), repeat.get_maximum())
            },
            None => self.matches_string_without_repeat(string_pointer)
        }
    }

    fn matches_string_without_repeat(&self, string_pointer: &mut StringPointer) -> bool {
        let matches = self.regex_type.matches_string(string_pointer);
        match (matches, &self.next) {
            (true, Some(next_regex)) => next_regex.matches_string(string_pointer),
            (true, None) => true,
            _ => false
        }
    }

    fn matches_string_with_repeat(&self, string_pointer: &mut StringPointer, min_repeat: Option<usize>, max_repeat: Option<usize>) -> bool {
        let min = match min_repeat {
            Some(min) => min,
            None => 0
        };

        let minimum_matches = self.matches_minimum_repeats(string_pointer, min);
        if !minimum_matches { return false }
        string_pointer.set_checkpoint();

        let remaining_matches = match &self.next {
            Some(next_regex) => next_regex.matches_string(string_pointer),
            None => true
        };

        if !remaining_matches {
            string_pointer.return_to_checkpoint().unwrap()
        }

        let mut counter = min;

        while !Self::counter_at_max(counter, &max_repeat) {
            let matches = self.regex_type.matches_string(string_pointer);
            string_pointer.set_checkpoint();
            let remain_matches = match (matches, &self.next) {
                (true, Some(next_regex)) => next_regex.matches_string(string_pointer),
                (true, None) => true,
                _ => false
            };

            if remain_matches {
                return true
            } else {
                string_pointer.return_to_checkpoint().unwrap()
            }
        }

        false
    }

    fn matches_minimum_repeats(&self, string_pointer: &mut StringPointer, min: usize) -> bool {
        for _ in 0..min {
            if !self.regex_type.matches_string(string_pointer) {
                return false
            }
        }

        true
    }

    fn counter_at_max(counter: usize, max_repeat: &Option<usize>) -> bool {
        match max_repeat {
            Some(value) => &counter == value,
            None => false
        }
    }
}