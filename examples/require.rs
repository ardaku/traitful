#[derive(Clone)]
pub struct MyStruct;

pub trait Clonable:
    clonable_traitful_require__::Require<clonable_traitful_require__::traits::Clone>
{
    /// Some docs
    fn uwu(&self);

    /// Clone and box
    // TODO: `fn clone_boxed(&self: impl Clone + 'static) -> Box<dyn Clonable>`
    fn clone_boxed(&self) -> Box<dyn Clonable> {
        <Self as clonable_traitful_require__::Require<
            clonable_traitful_require__::traits::Clone,
        >>::clone_boxed_traitful_require__(self)
    }
}

impl Clonable for MyStruct {
    fn uwu(&self) {
        println!("uwu");
    }
}

mod clonable_traitful_require__ {
    pub trait Require<Clone> {
        #[doc(hidden)]
        fn clone_boxed_traitful_require__(&self) -> Box<dyn super::Clonable>;
    }

    mod impl_ {
        use super::super::*;

        impl<T> super::Require<super::traits::Clone> for T
        where
            T: Clonable + Clone + 'static,
        {
            fn clone_boxed_traitful_require__(&self) -> Box<dyn Clonable> {
                Box::new(self.clone())
            }
        }
    }

    pub(super) mod traits {
        pub struct Clone;
    }
}

fn main() {
    let a: Box<dyn Clonable> = Box::new(MyStruct);

    a.uwu();

    let b = a.clone_boxed();

    b.uwu();
}
