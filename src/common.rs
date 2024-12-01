mod bound_generics_type;
mod generic_arg;
mod obfuscate_generics;
mod obfuscate_supertraits;
mod remove_arg_patterns;
mod unwrap;

pub(crate) use self::{
    bound_generics_type::BoundGenericsType, generic_arg::generic_arg,
    obfuscate_generics::obfuscate_generics,
    obfuscate_supertraits::obfuscate_supertraits,
    remove_arg_patterns::remove_arg_patterns, unwrap::unwrap,
};
