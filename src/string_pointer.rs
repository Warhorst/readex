use std::fmt::Formatter;

pub type Result = std::result::Result<String, StringPointerError>;

pub struct StringPointer {
    index: usize,
    string: String,
    history: Vec<usize>,
}

/// A string with a pointer to its current position and a history.
/// It allows to take slices from the string, beginning at its current position.
/// The history enables an easy reset to a former location.
impl StringPointer {
    pub fn from(string: &str) -> Self {
        StringPointer {
            index: 0,
            string: String::from(string),
            history: vec![0],
        }
    }

    /// Take the next <amount> chars from the string and give them back.
    /// The index is adapted and a new history entry is created.
    pub fn take_next(&mut self, amount: usize) -> Result {
        if self.max_index() < self.index + amount {
            return Err(StringPointerError::SizeExceeded)
        }

        let result = String::from(&self.string[self.index..(amount + self.index)]);
        self.index += amount;
        self.history.push(self.index);
        Ok(result)
    }

    /// Resets the pointer to its former position and removes the last history entry.
    pub fn set_back(&mut self) {
        if self.history.len() > 1 {
            self.history.pop().unwrap();
            self.index = *self.history.last().unwrap()
        }
    }

    /// Returns if the pointer points to the end of the string.
    pub fn at_the_end(&self) -> bool {
        self.index == self.string.len()
    }

    fn max_index(&self) -> usize {
        self.string.len()
    }
}

#[derive(Debug, PartialEq)]
pub enum StringPointerError {
    SizeExceeded
}

impl std::error::Error for StringPointerError {}

impl std::fmt::Display for StringPointerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SizeExceeded => write!(f, "Size of StringPointer exceeded with given next amount!")
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::string_pointer::{StringPointer, StringPointerError};

    #[test]
    pub fn success_take_next() {
        let mut string_pointer = StringPointer::from("foobarbaz");

        let foo = string_pointer.take_next(3).unwrap();
        let bar = string_pointer.take_next(3).unwrap();
        let baz = string_pointer.take_next(3).unwrap();

        assert_eq!("foo".to_string(), foo);
        assert_eq!("bar".to_string(), bar);
        assert_eq!("baz".to_string(), baz);
        assert_eq!(vec![0, 3, 6, 9], string_pointer.history);
        assert_eq!(9, string_pointer.index)
    }

    #[test]
    pub fn success_set_back() {
        let mut string_pointer = StringPointer::from("foobarbaz");
        let foo = string_pointer.take_next(3).unwrap();
        let bar = string_pointer.take_next(3).unwrap();
        assert_eq!(vec![0, 3, 6], string_pointer.history);
        assert_eq!(6, string_pointer.index);

        string_pointer.set_back();
        assert_eq!(vec![0, 3], string_pointer.history);
        assert_eq!(3, string_pointer.index);

        string_pointer.set_back();
        assert_eq!(vec![0], string_pointer.history);
        assert_eq!(0, string_pointer.index)
    }

    #[test]
    pub fn success_at_the_end() {
        let string = "foo";
        let mut string_pointer = StringPointer::from(string);

        assert!(!string_pointer.at_the_end());
        string_pointer.take_next(string.len()).unwrap();
        assert!(string_pointer.at_the_end())
    }

    #[test]
    pub fn failure_size_exceded() {
        let string = "foo";
        let mut string_pointer = StringPointer::from(string);

        let result = string_pointer.take_next(string.len() + 1);

        match result {
            Err(error) => assert_eq!(StringPointerError::SizeExceeded, error),
            _ => panic!("Result not as expected!")
        }
    }
}