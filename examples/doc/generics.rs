use traitful::seal;

pub struct Newtype<T>(T);

impl<T> Generic<T> for Newtype<T> {}

#[seal(Newtype<T>)]
pub trait Generic<T> {}
