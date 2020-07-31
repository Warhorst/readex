use crate::matcher::Matcher;

pub struct Any;

/// A matcher that returns true for any given char.
impl Matcher for Any {
    fn matches(&self, _string: String) -> bool {
        true
    }

    fn checked_string_length(&self) -> usize {
        1
    }
}

#[cfg(test)]
mod tests {
    use crate::matcher::any::Any;
    use crate::matcher::Matcher;

    #[test]
    fn success_matches() {
        let string = String::from("f");
        let matcher = Any;

        assert!(matcher.matches(string))
    }
}

