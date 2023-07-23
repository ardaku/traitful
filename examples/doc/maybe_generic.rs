use traitful::seal;

pub struct Newtype<T>(T);

impl<T> MaybeGeneric for Newtype<T> {}

pub struct UnitStruct;

impl MaybeGeneric for UnitStruct {}

#[seal(for<T> Newtype<T>, UnitStruct)]
pub trait MaybeGeneric {}
