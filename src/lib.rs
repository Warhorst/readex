pub mod regex;
pub mod matcher;
pub mod repeat;
pub mod string_pointer;

#[cfg(test)]
mod tests {
    use crate::matcher::StupidMatcher;
    use crate::regex::Regex;

    #[test]
    fn it_works() {
        let regex = Regex::matcher(StupidMatcher);
        let matches = regex.matches("foo");
        assert!(matches)
    }
}
