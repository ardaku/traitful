mod path {
    use super::SealedTrait;

    #[allow(dead_code)]
    pub struct MyStruct;

    impl SealedTrait for MyStruct {}
}

use traitful::seal;

#[seal(path::MyStruct)]
pub trait SealedTrait {}

fn main() {}
