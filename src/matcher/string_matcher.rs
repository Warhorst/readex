use crate::matcher::Matcher;
use crate::string_pointer::StringPointer;

/// Matcher that matches a given, exact String.
pub struct StringMatcher {
    string: String
}

impl StringMatcher {
    pub fn new(string: String) -> Self {
        StringMatcher {
            string
        }
    }
}

impl Matcher for StringMatcher {
    fn matches(&self, mut string_pointer: StringPointer) -> bool {
        let other_string = string_pointer.take_next(self.string.len());
        other_string == self.string
    }
}

#[cfg(test)]
mod tests {
    use crate::matcher::Matcher;
    use crate::matcher::string_matcher::StringMatcher;
    use crate::string_pointer::StringPointer;

    #[test]
    pub fn success_matches() {
        let string_pointer = StringPointer::from("foo");
        let matcher = StringMatcher::new("foo".to_string());
        assert!(matcher.matches(string_pointer))
    }

    #[test]
    pub fn success_matches_not() {
        let string_pointer = StringPointer::from("foo");
        let matcher = StringMatcher::new("bar".to_string());
        assert!(!matcher.matches(string_pointer))
    }
}