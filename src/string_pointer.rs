use std::fmt::Formatter;

use crate::string_pointer::StringPointerError::NoCheckpointToReturn;

pub type Result<T> = std::result::Result<T, StringPointerError>;

/// A string with a pointer to its current position and a history.
/// It allows to take slices from the string, beginning at its current position.
/// The check points enable an easy reset to a former location.
pub struct StringPointer {
    index: usize,
    string: String,
    check_points: Vec<usize>,
}

impl StringPointer {
    pub fn from(string: &str) -> Self {
        StringPointer {
            index: 0,
            string: String::from(string),
            check_points: vec![],
        }
    }

    /// Take the next <amount> chars from the string and give them back.
    /// The index is adapted.
    pub fn take_next(&mut self, amount: usize) -> Result<String> {
        if self.max_index() < self.index + amount {
            return Err(StringPointerError::SizeExceeded)
        }
        let result = String::from(&self.string[self.index..(amount + self.index)]);
        self.index += amount;
        Ok(result)
    }

    /// Sets the current index as a checkpoint.
    pub fn set_checkpoint(&mut self) {
        if !self.check_points.contains(&self.index) {
            self.check_points.push(self.index)
        }
    }

    /// Resets the pointer to its former checkpoint, removing it in the process.
    pub fn return_to_checkpoint(&mut self) -> Result<()> {
        if self.check_points.len() >= 1 {
            let new_index = self.check_points.pop().unwrap();
            self.index = new_index;
            Ok(())
        } else {
            Err(NoCheckpointToReturn)
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
    SizeExceeded,
    NoCheckpointToReturn,
}

impl std::error::Error for StringPointerError {}

impl std::fmt::Display for StringPointerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SizeExceeded => write!(f, "Size of StringPointer exceeded with given next amount!"),
            Self::NoCheckpointToReturn => writeln!(f, "There is no checkpoint set to return to!")
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::string_pointer::{StringPointer, StringPointerError};
    use crate::string_pointer::StringPointerError::{NoCheckpointToReturn, SizeExceeded};

    #[test]
    pub fn success_take_next() {
        let mut string_pointer = StringPointer::from("foobarbaz");

        let foo = string_pointer.take_next(3).unwrap();
        let bar = string_pointer.take_next(3).unwrap();
        let baz = string_pointer.take_next(3).unwrap();

        assert_eq!("foo".to_string(), foo);
        assert_eq!("bar".to_string(), bar);
        assert_eq!("baz".to_string(), baz);
        assert_eq!(9, string_pointer.index)
    }

    #[test]
    pub fn success_set_checkpoint() {
        let mut string_pointer = StringPointer::from("foo");

        let empty_vec: Vec<usize> = vec![];
        assert_eq!(empty_vec, string_pointer.check_points);

        string_pointer.set_checkpoint();
        assert_eq!(vec![0], string_pointer.check_points)
    }

    #[test]
    pub fn success_checkpoint_not_set_twice() {
        let mut string_pointer = StringPointer::from("foo");

        string_pointer.set_checkpoint();
        string_pointer.set_checkpoint();

        assert_eq!(vec![0], string_pointer.check_points);
    }

    #[test]
    pub fn success_return_to_checkpoint() {
        let mut string_pointer = StringPointer::from("foobar");

        let foo = string_pointer.take_next(3).unwrap();
        string_pointer.set_checkpoint();
        let bar = string_pointer.take_next(3).unwrap();
        assert_eq!(vec![3], string_pointer.check_points);
        assert_eq!(6, string_pointer.index);

        let result = string_pointer.return_to_checkpoint();
        let empty_vec: Vec<usize> = vec![];
        assert_eq!(result, Ok(()));
        assert_eq!(empty_vec, string_pointer.check_points);
        assert_eq!(3, string_pointer.index)
    }

    #[test]
    pub fn failure_return_to_checkpoint() {
        let mut string_pointer = StringPointer::from("foo");

        let result = string_pointer.return_to_checkpoint();

        assert_eq!(Err(NoCheckpointToReturn), result)
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

        assert_eq!(Err(SizeExceeded), result)
    }
}