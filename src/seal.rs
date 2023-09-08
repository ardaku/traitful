use proc_macro2::TokenStream;
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    token::{Comma, Gt, Lt},
    AngleBracketedGenericArguments, Expr, ExprPath, GenericArgument,
    GenericParam, Generics, ItemTrait, Path, PathArguments, PathSegment,
    Result, Token, TraitBound, TraitBoundModifier, Type, TypeParam,
    TypeParamBound, TypePath,
};

use crate::common::{self, BoundGenericsType};

struct AttrParams {
    types: Punctuated<BoundGenericsType, Token![,]>,
}

impl Parse for AttrParams {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        Ok(Self {
            types: Punctuated::parse_terminated(input)?,
        })
    }
}

pub(super) fn seal(
    attr: TokenStream,
    item: TokenStream,
) -> Result<TokenStream> {
    let trait_ = item;
    let mut trait_: ItemTrait = syn::parse2(trait_)?;
    let trait_ident = trait_.ident.clone();
    let trait_generics = trait_.generics.clone();
    let seal_ident = proc_macro2::Ident::new(
        &format!("{trait_ident}_traitful_seal__"),
        proc_macro2::Span::call_site(),
    );

    trait_.supertraits.push(TypeParamBound::Trait(TraitBound {
        paren_token: None,
        modifier: TraitBoundModifier::None,
        lifetimes: None,
        path: Path {
            leading_colon: None,
            segments: {
                let mut path = Punctuated::new();
                let angle =
                    trait_generics.lt_token.zip(trait_generics.gt_token);
                let arguments = if let Some((lt_token, gt_token)) = angle {
                    let mut args = Punctuated::new();

                    for param in trait_generics.params.clone() {
                        args.push(match param {
                            GenericParam::Lifetime(lifetime) => {
                                GenericArgument::Lifetime(lifetime.lifetime)
                            }
                            GenericParam::Type(type_) => {
                                GenericArgument::Type(Type::Path(TypePath {
                                    qself: None,
                                    path: Path {
                                        leading_colon: None,
                                        segments: Punctuated::from_iter([
                                            PathSegment {
                                                ident: type_.ident,
                                                arguments: PathArguments::None,
                                            },
                                        ]),
                                    },
                                }))
                            }
                            GenericParam::Const(const_) => {
                                GenericArgument::Const(Expr::Path(ExprPath {
                                    attrs: Vec::new(),
                                    qself: None,
                                    path: Path {
                                        leading_colon: None,
                                        segments: Punctuated::from_iter([
                                            PathSegment {
                                                ident: const_.ident,
                                                arguments: PathArguments::None,
                                            },
                                        ]),
                                    },
                                }))
                            }
                        });
                    }

                    PathArguments::AngleBracketed(
                        AngleBracketedGenericArguments {
                            colon2_token: None,
                            lt_token,
                            args,
                            gt_token,
                        },
                    )
                } else {
                    PathArguments::None
                };

                path.push(PathSegment {
                    ident: proc_macro2::Ident::new(
                        "self",
                        proc_macro2::Span::call_site(),
                    ),
                    arguments: PathArguments::None,
                });
                path.push(PathSegment {
                    ident: seal_ident.clone(),
                    arguments: PathArguments::None,
                });
                path.push(PathSegment {
                    ident: proc_macro2::Ident::new(
                        "Seal",
                        proc_macro2::Span::call_site(),
                    ),
                    arguments,
                });

                path
            },
        },
    }));

    let attr: AttrParams = syn::parse2(attr)?;
    let mut stream = TokenStream::new();
    let mut trait_args: Punctuated<GenericArgument, Comma> = Punctuated::new();

    for trait_generic in trait_generics.params.iter().cloned() {
        trait_args.push(common::generic_arg(trait_generic));
    }

    for param in attr.types.into_iter() {
        let type_ = &param.type_;
        let mut generic_params = Generics {
            lt_token: Some(Lt {
                spans: [proc_macro2::Span::call_site()],
            }),
            params: Punctuated::new(),
            gt_token: Some(Gt {
                spans: [proc_macro2::Span::call_site()],
            }),
            where_clause: None,
        };

        if let Some(generics) = param.bound_generics {
            for param in generics.params {
                generic_params.params.push(param);
            }
        }

        stream.extend(quote::quote! {
            impl #generic_params #trait_generics super::Seal<#trait_args>
                for #type_ {}
        });
    }

    let seal_generics = {
        let mut seal_generics = trait_generics.clone();

        seal_generics.params.push(GenericParam::Type(TypeParam {
            attrs: Vec::new(),
            ident: proc_macro2::Ident::new(
                "T_traitful_seal__",
                proc_macro2::Span::call_site(),
            ),
            colon_token: None,
            bounds: Punctuated::new(),
            eq_token: None,
            default: None,
        }));

        seal_generics
    };

    Ok(quote::quote! {
        #trait_

        #[doc(hidden)]
        pub(crate) mod #seal_ident {
            #[doc(hidden)]
            pub trait Seal #trait_generics: self::Sealed<#trait_args> {}
            #[doc(hidden)]
            pub trait Sealed #trait_generics {}

            impl #seal_generics self::Sealed<#trait_args>
                for T_traitful_seal__
                where T_traitful_seal__: super::#trait_ident<#trait_args> {}

            mod impl_traitful_seal__ {
                pub use super::super::*;

                #stream
            }
        }
    })
}
