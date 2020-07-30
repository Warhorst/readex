pub mod regex;
pub mod matcher;
pub mod repeat;
pub mod string_pointer;
pub mod expression;

#[cfg(test)]
mod tests {
    use crate::matcher::StupidMatcher;
    use crate::regex::Regex;

    #[test]
    fn it_works() {
        let regex = Regex::for_matcher(StupidMatcher);
        let matches = regex.matches_string("foo");
        assert!(matches)
    }
}
