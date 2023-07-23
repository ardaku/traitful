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
/// implementation enforces that the trait is implemented for the exact set of
/// types specified, no more and no less.
///
/// ```rust
/// # mod meow {
#[doc = include_str!("../examples/doc/meow.rs")]
/// # }
/// ```
/// 
/// ## Generics
/// You can also seal traits with generics
/// ```rust
/// # mod generics {
#[doc = include_str!("../examples/doc/generics.rs")]
/// # }
/// ```
/// 
/// ## Generics with `for` syntax
/// You can also seal traits to types with generics without adding them to the
/// trait by using the `for` syntax.
/// ```rust
/// # mod maybe_generic {
#[doc = include_str!("../examples/doc/maybe_generic.rs")]
/// # }
/// ```
/// 
/// [Rust API Guidelines]: https://rust-lang.github.io/api-guidelines/future-proofing.html#sealed-traits-protect-against-downstream-implementations-c-sealed
#[proc_macro_attribute]
pub fn seal(attr: TokenStream, item: TokenStream) -> TokenStream {
    self::seal::seal(attr, item)
}

/// Make a trait an extension of a struct's functionality.
///
/// # Extension Traits
/// Extension traits are documented in [RFC-0445] as a way to extend
/// functionality of structs defined in upstream crates.  The new methods are
/// called "Extension methods".  This implementation enforces that extension
/// traits are sealed.
///
/// ```rust
/// # mod duration {
#[doc = include_str!("../examples/doc/duration.rs")]
/// # }
/// ```
/// 
/// ## Generics
/// You can also make extension traits on types with generics by adding the
/// generic parameter to the trait definition:
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
