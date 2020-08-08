pub mod regex;
pub mod matcher;
pub mod repeat;
pub mod string_pointer;

#[cfg(test)]
mod tests {
    use crate::matcher::any::Any;
    use crate::matcher::string::Str;
    use crate::regex::Regex;
    use crate::repeat::times::Times;

    #[test]
    fn it_works() {
        let regex_one = Regex::matcher(Str::new("foo"))
            .followed_by(Regex::matcher(Any))
            .followed_by(Regex::matcher(Str::new("bar")))
            .followed_by(Regex::matcher(Any))
            .followed_by(Regex::matcher(Str::new("baz")));

        let regex_two = Regex::matcher(Str::new("foo"))
            .followed_by(Regex::matcher(Any)
                .followed_by(Regex::matcher(Str::new("bar"))
                    .followed_by(Regex::matcher(Any)
                        .followed_by(Regex::matcher(Str::new("baz"))))));

        let regex_three = Regex::matcher(Str::new("foo"))
            .followed_by(Regex::matcher(Str::new("A")).that_repeats(Times::new(3)))
            .followed_by(Regex::matcher(Str::new("bar")));

        let regex_four = Regex::matcher(Str::new("A")).that_repeats(Times::new(3));


        assert!(regex_one.matches("foo bar baz"));
        assert!(regex_two.matches("foo bar baz"));
        // assert!(regex_three.matches("fooAAAbar"))
        assert!(regex_four.matches("AAA"))
    }
}
