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
}
