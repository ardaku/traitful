use proc_macro2::{Span, TokenStream};
use syn::{
    parse::Error,
    punctuated::Punctuated,
    token::{Brace, For, Gt, Impl, Lt},
    AngleBracketedGenericArguments, GenericParam, Ident, ImplItem, ImplItemFn,
    ItemImpl, ItemTrait, Path, PathArguments, PathSegment, Result, TraitItem,
    Type, TypeParam, TypePath, Visibility,
};

use crate::common::{self, BoundGenericsType};

pub(super) fn extend(
    attr: TokenStream,
    item: TokenStream,
) -> Result<TokenStream> {
    let mut trait_: ItemTrait = syn::parse2(item)?;

    let type_: BoundGenericsType = if attr.is_empty() {
        if trait_.generics.where_clause.is_some() {
            return Err(Error::new(
                Span::call_site(),
                "`where` clause not supported for `#[extend]` inference",
            ));
        }

        let ident = Ident::new("T_traitful_extend__", Span::call_site());
        let mut params = trait_.generics.params.clone();

        params.push(GenericParam::Type(TypeParam {
            attrs: Vec::new(),
            ident: ident.clone(),
            colon_token: trait_.colon_token.clone(),
            bounds: trait_.supertraits.clone(),
            eq_token: None,
            default: None,
        }));

        BoundGenericsType {
            bound_generics: Some(params.into()),
            type_: Type::Path(TypePath {
                qself: None,
                path: Path {
                    leading_colon: None,
                    segments: Punctuated::from_iter([PathSegment {
                        ident,
                        arguments: PathArguments::None,
                    }]),
                },
            }),
        }
    } else {
        syn::parse2(attr)?
    };
    let generics = type_
        .bound_generics
        .clone()
        .map(From::from)
        .unwrap_or_default();
    let mut impl_ = ItemImpl {
        attrs: Vec::new(),
        defaultness: None,
        unsafety: None,
        impl_token: Impl {
            span: Span::call_site(),
        },
        generics,
        trait_: None,
        self_ty: Box::new(type_.type_.clone()),
        brace_token: Brace::default(),
        items: Vec::new(),
    };
    let trait_ = {
        let ident = trait_.ident.clone();

        impl_.trait_ = Some((
            None,
            Path {
                leading_colon: None,
                segments: Punctuated::from_iter([PathSegment {
                    ident,
                    arguments: PathArguments::AngleBracketed(
                        AngleBracketedGenericArguments {
                            colon2_token: None,
                            lt_token: Lt::default(),
                            args: {
                                let mut args = Punctuated::new();

                                for generic_param in
                                    trait_.generics.params.iter().cloned()
                                {
                                    args.push(common::generic_arg(
                                        generic_param,
                                    ))
                                }

                                args
                            },
                            gt_token: Gt::default(),
                        },
                    ),
                }]),
            },
            For {
                span: Span::call_site(),
            },
        ));

        for item in trait_.items.iter_mut() {
            let TraitItem::Fn(ref mut fn_) = item else {
                continue;
            };
            let Some(block) = fn_.default.take() else {
                return Err(Error::new(
                    Span::call_site(),
                    "Method block required",
                ));
            };
            let sig = fn_.sig.clone();

            common::remove_arg_patterns(&mut fn_.sig);

            impl_.items.push(ImplItem::Fn(ImplItemFn {
                attrs: Vec::new(),
                vis: Visibility::Inherited,
                defaultness: None,
                sig,
                block,
            }));
        }

        trait_
    };

    let params = impl_.generics.params.clone();

    impl_.generics.params.extend(trait_.generics.params.clone());

    let type_ = type_.type_;

    Ok(quote::quote! {
        #[::traitful::seal(for<#params> #type_)]
        #trait_

        #impl_
    })
}
