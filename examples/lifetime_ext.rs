use traitful::extend;

pub trait Lifetime<'a> {}

#[extend(for<T: Lifetime<'a>> T)]
pub trait ExtensionA<'a, U>: Lifetime<'a> {}

#[extend]
pub trait ExtensionB<'b>: Lifetime<'b> {}

fn main() {}
