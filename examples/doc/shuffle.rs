use traitful::extend;

/// Extend mutable slices with new `shuffle()` method
#[extend(&mut [T])]
pub trait Shuffle<T> {
    /// Randomly shuffle a slice with `rand` providing random indices from 0 up
    /// to a maximum value.
    fn shuffle(&mut self, rand: &mut dyn FnMut(usize) -> usize) {
        for i in (1..self.len()).rev() {
            self.swap(i, rand(i));
        }
    }
}
