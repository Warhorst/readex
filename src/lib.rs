pub mod regex;
pub mod matcher;
pub mod repeat;
pub mod string_pointer;

#[cfg(test)]
mod tests {
    use std::sync::mpsc::RecvTimeoutError::Timeout;

    use crate::matcher::any::Any;
    use crate::matcher::string::Str;
    use crate::regex::Regex;
    use crate::repeat::times::Times;
    use crate::repeat::zero_to_infinity::ZeroToInfinity;

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
            .followed_by(Regex::matcher(Str::new("A")).that_repeats(ZeroToInfinity))
            .followed_by(Regex::matcher(Str::new("bar")));

        let regex_four = Regex::matcher(Str::new("A")).that_repeats(ZeroToInfinity);

        println!("{}", regex_three);


        // assert!(regex_one.matches("foo bar baz"));
        // assert!(regex_two.matches("foo bar baz"));
        assert!(regex_three.matches("fooAAAAbar"));
        // assert!(regex_four.matches("AAAnAA"))
    }
}
