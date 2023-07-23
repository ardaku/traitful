use traitful::seal;

pub struct MyStruct;

impl SealedTrait for MyStruct {}

#[seal(MyStruct)]
pub trait SealedTrait: Sized + 'static {}

fn main() {}
