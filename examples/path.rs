mod path {
    use super::SealedTrait;

    pub struct MyStruct;

    impl SealedTrait for MyStruct {}
}

use traitful::seal;

#[seal(path::MyStruct)]
pub trait SealedTrait {}

fn main() {}
