use std::num::Wrapping;

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

fn main() {
    let mut array = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    let sequence = 0xb5ad4eceda1ce2a9u64;
    let mut x = Wrapping(0);
    let mut w = Wrapping(0);
    let mut rand = |upto: usize| {
        let upto = u64::try_from(upto).unwrap();

        x *= x;
        w += Wrapping(sequence);
        x += w;
        x = (x >> 32) | (x << 32);

        usize::try_from(x.0 % (upto + 1)).unwrap()
    };

    for _ in 0..5 {
        array.as_mut_slice().shuffle(&mut rand);

        println!("{array:?}");

        array.sort();
        println!("{array:?}");
    }
}
