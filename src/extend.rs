use proc_macro2::{Span, TokenStream};
use syn::{
    parse::Error,
    punctuated::Punctuated,
    token::{Brace, For, Gt, Impl, Lt},
    AngleBracketedGenericArguments, Generics, ImplItem, ImplItemFn, ItemImpl,
    ItemTrait, Path, PathArguments, PathSegment, Result, TraitItem, Type,
    Visibility,
};

use crate::common;

pub(super) fn extend(
    attr: TokenStream,
    item: TokenStream,
) -> Result<TokenStream> {
    let type_: Type = syn::parse2(attr)?;
    let mut impl_ = ItemImpl {
        attrs: Vec::new(),
        defaultness: None,
        unsafety: None,
        impl_token: Impl {
            span: Span::call_site(),
        },
        generics: Generics::default(),
        trait_: None,
        self_ty: Box::new(type_.clone()),
        brace_token: Brace::default(),
        items: Vec::new(),
    };
    let trait_ = {
        let mut trait_: ItemTrait = syn::parse2(item)?;
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

    impl_.generics = trait_.generics.clone();

    Ok(quote::quote! {
        #[::traitful::seal(#type_)]
        #trait_

        #impl_
    })
}
