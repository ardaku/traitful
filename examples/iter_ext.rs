use traitful::extend;

/// Extend the `Iterator` trait with new `all_eq()` method
#[extend(for<I: Iterator> I)]
pub trait IteratorExt: Iterator {
    /// Return true if all elements of the iterator are equal.
    fn all_eq(mut self) -> bool
    where
        Self::Item: PartialEq,
        Self: Sized,
    {
        let Some(first) = self.next() else {
            return true;
        };

        self.all(|x| x == first)
    }
}

fn main() {
    assert!([4, 4, 4].into_iter().all_eq());
    assert!(![4, 5, 4].into_iter().all_eq());
}
