mod bound_generics_type;
mod generic_arg;
mod remove_arg_patterns;
mod unwrap;

pub(crate) use self::{
    bound_generics_type::BoundGenericsType, generic_arg::generic_arg,
    remove_arg_patterns::remove_arg_patterns, unwrap::unwrap,
};
