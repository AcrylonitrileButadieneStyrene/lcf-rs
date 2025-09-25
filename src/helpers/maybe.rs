#[derive(Clone, Debug, Default)]
pub struct Maybe<T> {
    value: T,
    written: bool,
}

impl<T> Maybe<T> {
    pub fn finish(self) -> Option<T> {
        if self.written { Some(self.value) } else { None }
    }
}

impl<T> std::ops::Deref for Maybe<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T> std::ops::DerefMut for Maybe<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.written = true;
        &mut self.value
    }
}

#[cfg(test)]
mod tests {
    #[derive(Debug, Default, PartialEq)]
    struct Struct(bool);

    #[test]
    fn none() {
        let field: super::Maybe<Struct> = Default::default();
        assert_eq!(field.finish(), None);
    }

    #[test]
    fn some_no_change() {
        let mut field: super::Maybe<Struct> = Default::default();
        field.0 = false;
        assert_eq!(field.finish(), Some(Struct(false)));
    }

    #[test]
    fn some_change() {
        let mut field: super::Maybe<Struct> = Default::default();
        field.0 = true;
        assert_eq!(field.finish(), Some(Struct(true)));
    }
}
