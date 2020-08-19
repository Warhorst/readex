use crate::matcher::Matcher;
use crate::string_pointer::StringPointer;

/// Matcher that matches a given, exact String.
pub struct Str {
    string: String
}

impl Str {
    pub fn new(string: &str) -> Self {
        Str {
            string: String::from(string)
        }
    }
}

impl Matcher for Str {
    fn matches(&self, mut string: String) -> bool {
        println!("{}", format!("Matching: {}", string));
        string == self.string
    }

    fn checked_string_length(&self) -> usize {
        self.string.len()
    }
}

#[cfg(test)]
mod tests {
    use crate::matcher::Matcher;
    use crate::matcher::string::Str;
    use crate::string_pointer::StringPointer;

    #[test]
    pub fn success_matches() {
        let string = String::from("foo");
        let matcher = Str::new("foo");
        assert!(matcher.matches(string))
    }

    #[test]
    pub fn success_matches_not() {
        let string = String::from("foo");
        let matcher = Str::new("bar");
        assert!(!matcher.matches(string))
    }
}