use ResultExt;

impl<T, E> ResultExt<T, E> for Result<T, E> {
    fn contains<U>(&self, x: &U) -> bool where U: PartialEq<T> {
        match *self {
            Ok(ref y) => x == y,
            Err(_) => false
        }
    }

    fn contains_err<F>(&self, f: &F) -> bool where F: PartialEq<E> {
        match *self {
            Ok(_) => false,
            Err(ref e) => f == e
        }
    }

    #[inline]
    fn map_or2<U, F: FnOnce(T) -> U>(self, f: F, default: U) -> U {
        self.map_or(default, f)
    }

    #[inline]
    fn map_or_else2<U, F: FnOnce(T) -> U, D: FnOnce(E) -> U>(self, f: F, default: D) -> U {
        self.map_or_else(default, f)
    }
}
