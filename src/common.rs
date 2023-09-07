use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use syn::Result;

pub(super) fn unwrap(result: Result<TokenStream2>) -> TokenStream {
    result.unwrap_or_else(|e| e.into_compile_error()).into()
}
