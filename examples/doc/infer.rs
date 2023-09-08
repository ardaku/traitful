use traitful::extend;

/// Extend the `Iterator` trait with new `all_eq()` method
#[extend]
pub trait IteratorExt: Iterator {
    /// Return true if all elements of the iterator are equal.
    fn all_eq(mut self) -> bool
    where
        Self::Item: PartialEq,
    {
        let Some(first) = self.next() else {
            return true;
        };

        self.all(|x| x == first)
    }
}
