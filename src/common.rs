use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use syn::{
    punctuated::Punctuated, Expr, ExprPath, GenericArgument, GenericParam,
    Path, PathArguments, PathSegment, Result, Type, TypePath,
};

/// Create a generic argument from a generic parameter
pub(crate) fn generic_arg(generic_param: GenericParam) -> GenericArgument {
    match generic_param {
        GenericParam::Lifetime(param) => {
            GenericArgument::Lifetime(param.lifetime)
        }
        GenericParam::Type(param) => {
            GenericArgument::Type(Type::Path(TypePath {
                qself: None,
                path: Path {
                    leading_colon: None,
                    segments: Punctuated::from_iter([PathSegment {
                        ident: param.ident,
                        arguments: PathArguments::None,
                    }]),
                },
            }))
        }
        GenericParam::Const(param) => {
            GenericArgument::Const(Expr::Path(ExprPath {
                attrs: Vec::new(),
                qself: None,
                path: Path {
                    leading_colon: None,
                    segments: Punctuated::from_iter([PathSegment {
                        ident: param.ident,
                        arguments: PathArguments::None,
                    }]),
                },
            }))
        }
    }
}

pub(super) fn unwrap(result: Result<TokenStream2>) -> TokenStream {
    result.unwrap_or_else(|e| e.into_compile_error()).into()
}
