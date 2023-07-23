use traitful::seal;

// Make it so that only cats and cat-people can meow
#[seal(Cat, CatPerson)]
pub trait Meow {
    fn meow() {
        println!("meow");
    }
}

pub struct Cat;

impl Meow for Cat {}

pub struct CatPerson;

// Will fail to compile if this line is commented out
impl Meow for CatPerson {}
