use traitful::seal;

pub struct Newtype<T>(T);

impl<T> MaybeGeneric for Newtype<T> {}

pub struct Other;

impl MaybeGeneric for Other {}

#[seal(for<T> Newtype<T>, Other)]
pub trait MaybeGeneric {}

fn main() {}
