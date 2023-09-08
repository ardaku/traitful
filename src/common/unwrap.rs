use proc_macro2::TokenStream;
use syn::Result;

/// Unwrap the result and properly handle the error
pub(crate) fn unwrap(result: Result<TokenStream>) -> proc_macro::TokenStream {
    result.unwrap_or_else(|e| e.into_compile_error()).into()
}
