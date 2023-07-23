use traitful::extend;

// Extend slices to be shuffled
#[extend]
pub impl<T> Shuffle for &mut [T] {
    /// Randomly shuffle a slice with `rand` providing random indices from 0 up
    /// to a maximum value.
    fn shuffle(&mut self, rand: &mut dyn FnMut(usize) -> usize) {
        for i in (1..self.len()).rev() {
            self.swap(i, rand(i))
        }
    }
}
