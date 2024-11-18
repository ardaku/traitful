use traitful::extend;

pub trait Lifetime<'a> {}

#[extend(for<T: Lifetime<'a>> T)]
pub trait MaybeGeneric<'a, U>: Lifetime<'a> {}

fn main() {}
