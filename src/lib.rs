mod r#impl;

/// Extension trait providing additional methods for `Result`.
pub trait ResultExt<T, E> {
    /// Returns `true` if the result is an [`Ok`] value containing the given value.
    ///
    /// # Examples
    ///
    /// ```
    /// use result_ext::ResultExt;
    ///
    /// let x: Result<u32, &str> = Ok(2);
    /// assert_eq!(x.contains(&2), true);
    ///
    /// let x: Result<u32, &str> = Ok(3);
    /// assert_eq!(x.contains(&2), false);
    ///
    /// let x: Result<u32, &str> = Err("Some error message");
    /// assert_eq!(x.contains(&2), false);
    /// ```
    #[must_use]
    fn contains<U>(&self, x: &U) -> bool where U: PartialEq<T>;

    /// Returns `true` if the result is an [`Err`] value containing the given value.
    ///
    /// # Examples
    ///
    /// ```
    /// use result_ext::ResultExt;
    ///
    /// let x: Result<u32, &str> = Ok(2);
    /// assert_eq!(x.contains_err(&"Some error message"), false);
    ///
    /// let x: Result<u32, &str> = Err("Some error message");
    /// assert_eq!(x.contains_err(&"Some error message"), true);
    ///
    /// let x: Result<u32, &str> = Err("Some other error message");
    /// assert_eq!(x.contains_err(&"Some error message"), false);
    /// ```
    #[must_use]
    fn contains_err<F>(&self, f: &F) -> bool where F: PartialEq<E>;

    /// Returns the result from applying the function `f` to the contained value if the result is [`Ok`],
    /// or returns provided `default` value if the result is [`Err`].
    ///
    /// The `f` argument of `map_or2` is only evaluated if the result is [`Ok`].
    /// The `default` argument of `map_or2` is always evaluated – even if the result is [`Ok`].
    /// Use [`map_or_else2`] to avoid evaluating the `default` argument.
    ///
    /// [`map_or_else2`]: ResultExt::map_or_else2
    ///
    /// # Examples
    ///
    /// ```
    /// use result_ext::ResultExt;
    ///
    /// let x: Result<_, &str> = Ok("foo");
    /// assert_eq!(x.map_or2(|v| v.len(), 23), 3);
    ///
    /// let x: Result<&str, _> = Err("bar");
    /// assert_eq!(x.map_or2(|v| v.len(), 23), 23);
    /// ```
    #[must_use]
    fn map_or2<U, F: FnOnce(T) -> U>(self, f: F, default: U) -> U;

    /// Returns the result from applying the function `f` to the contained value if the result is [`Ok`],
    /// or returns the result from applying the function `default` to the contained error if the result is [`Err`].
    ///
    /// The `f` argument of `map_or_else2` is only evaluated if the result is [`Ok`].
    /// The `default` argument of `map_or_else2` is only evaluated if the result is [`Err`].
    /// Use [`map_or2`] to always evaluate the `default` argument.
    ///
    /// [`map_or2`]: ResultExt::map_or2
    ///
    /// # Examples
    ///
    /// ```
    /// use result_ext::ResultExt;
    ///
    /// let k = 23;
    ///
    /// let x : Result<_, &str> = Ok("foo");
    /// assert_eq!(x.map_or_else2(|v| v.len(), |e| k * 2), 3);
    ///
    /// let x : Result<&str, _> = Err("bar");
    /// assert_eq!(x.map_or_else2(|v| v.len(), |e| k * 2), 46);
    /// ```
    #[must_use]
    fn map_or_else2<U, F: FnOnce(T) -> U, D: FnOnce(E) -> U>(self, f: F, default: D) -> U;
}
