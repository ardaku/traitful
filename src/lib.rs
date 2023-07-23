//! #### A collection of helper macros for trait patterns

#![doc(
    html_logo_url = "https://ardaku.github.io/mm/logo.svg",
    html_favicon_url = "https://ardaku.github.io/mm/icon.svg",
    html_root_url = "https://docs.rs/traitful"
)]
#![forbid(missing_docs)]
#![warn(
    anonymous_parameters,
    missing_copy_implementations,
    missing_debug_implementations,
    nonstandard_style,
    rust_2018_idioms,
    single_use_lifetimes,
    trivial_casts,
    trivial_numeric_casts,
    unreachable_pub,
    unused_extern_crates,
    unused_qualifications,
    variant_size_differences
)]

mod extend;
mod seal;

use proc_macro::TokenStream;

/// Seal a trait.
///
/// # Sealed Traits
/// Sealed traits are documented in the [Rust API Guidelines] as a way to
/// prevent downstream implementations while providing a public trait API.  This
/// implementation is a way of enforcing that the trait is only implemented for
/// the set of types specified.  It also enforces that the sealed trait is
/// implemented for every type in the set specified.
///
/// ```rust
/// # mod meow {
#[doc = include_str!("../examples/doc/meow.rs")]
/// # }
/// ```
/// 
/// [Rust API Guidelines]: https://rust-lang.github.io/api-guidelines/future-proofing.html#sealed-traits-protect-against-downstream-implementations-c-sealed
#[proc_macro_attribute]
pub fn seal(attr: TokenStream, item: TokenStream) -> TokenStream {
    self::seal::seal(attr, item)
}

/// Extend a struct with new functionality.
///
/// # Extension Traits
/// Extension traits are documented in [RFC-0445] as a way to extend
/// functionality of structs defined in upstream crates.  The new methods are
/// called "Extension methods".
///
/// ```rust
/// # mod shuffle {
#[doc = include_str!("../examples/doc/shuffle.rs")]
/// # }
/// ```
/// 
/// [RFC-0445]: https://rust-lang.github.io/rfcs/0445-extension-trait-conventions.html
#[proc_macro_attribute]
pub fn extend(attr: TokenStream, item: TokenStream) -> TokenStream {
    self::extend::extend(attr, item)
}
