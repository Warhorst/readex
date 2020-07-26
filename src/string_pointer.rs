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
    pub fn take_next(&mut self, amount: usize) -> String {
        let result = String::from(&self.string[self.index..(amount + self.index)]);
        self.index += amount;
        self.history.push(self.index);
        result
    }

    /// Resets the pointer to its former position and removes the last history entry.
    pub fn set_back(&mut self) {
        self.history.pop().unwrap();
        self.index = *self.history.last().unwrap()
    }

    /// Returns if the pointer points to the end of the string.
    pub fn at_the_end(&self) -> bool {
        self.index == self.string.len()
    }
}

#[cfg(test)]
mod tests {
    use crate::string_pointer::StringPointer;

    #[test]
    pub fn success_take_next() {
        let mut string_pointer = StringPointer::from("foobarbaz");

        let foo = string_pointer.take_next(3);
        let bar = string_pointer.take_next(3);
        let baz = string_pointer.take_next(3);

        assert_eq!("foo".to_string(), foo);
        assert_eq!("bar".to_string(), bar);
        assert_eq!("baz".to_string(), baz);
        assert_eq!(vec![0, 3, 6, 9], string_pointer.history);
        assert_eq!(9, string_pointer.index)
    }

    #[test]
    pub fn success_set_back() {
        let mut string_pointer = StringPointer::from("foobarbaz");
        let foo = string_pointer.take_next(3);
        let bar = string_pointer.take_next(3);
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
        string_pointer.take_next(string.len());
        assert!(string_pointer.at_the_end())
    }
}