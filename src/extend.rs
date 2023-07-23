use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::ToTokens;
use syn::{
    parse::{Error, Parse, ParseStream},
    punctuated::Punctuated,
    token::Semi,
    ImplItem, ItemImpl, Result, TraitItem, TraitItemConst, TraitItemFn,
    TraitItemMacro, TraitItemType, Visibility,
};

struct Item {
    vis: Visibility,
    impl_: ItemImpl,
}

impl Parse for Item {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        Ok(Self {
            vis: Visibility::parse(input)?,
            impl_: ItemImpl::parse(input)?,
        })
    }
}

pub(super) fn extend(attr: TokenStream, item: TokenStream) -> TokenStream {
    if !attr.is_empty() {
        return Error::new(
            Span::call_site(),
            "No attribute parameters expected",
        )
        .into_compile_error()
        .into();
    }

    let item = syn::parse_macro_input!(item as Item);
    let vis = item.vis;
    let impl_ = item.impl_;
    let block = impl_.clone();
    let attrs = {
        let mut attrs = proc_macro2::TokenStream::new();

        for attr in impl_.attrs {
            attr.to_tokens(&mut attrs);
        }

        attrs
    };
    let unsafety = impl_.unsafety;
    let self_ty = impl_.self_ty;
    let generics = impl_.generics;
    let Some(trait_) = impl_.trait_ else {
        return Error::new(Span::call_site(), "Missing `for <type>`")
            .into_compile_error()
            .into();
    };
    let ident = {
        if trait_.0.is_some() {
            return Error::new(Span::call_site(), "Unexpected `!`")
                .into_compile_error()
                .into();
        }

        let path = trait_.1;

        if path.leading_colon.is_some() {
            return Error::new(Span::call_site(), "Unexpected leading `::`")
                .into_compile_error()
                .into();
        }

        let mut path = path.segments.into_iter();
        let Some(segment) = path.next() else {
            return Error::new(Span::call_site(), "Empty path")
                .into_compile_error()
                .into();
        };

        if path.next().is_some() {
            return Error::new(
                Span::call_site(),
                "Path contains more than one identifier",
            )
            .into_compile_error()
            .into();
        }

        segment.ident
    };
    let mut items = proc_macro2::TokenStream::new();

    for item in impl_.items {
        let item = match item {
            ImplItem::Const(const_) => TraitItem::Const(TraitItemConst {
                attrs: const_.attrs,
                const_token: const_.const_token,
                ident: const_.ident,
                generics: const_.generics,
                colon_token: const_.colon_token,
                ty: const_.ty,
                default: None,
                semi_token: const_.semi_token,
            }),
            ImplItem::Fn(fn_) => TraitItem::Fn(TraitItemFn {
                attrs: fn_.attrs,
                sig: fn_.sig,
                default: None,
                semi_token: Some(Semi {
                    spans: [Span::call_site()],
                }),
            }),
            ImplItem::Type(type_) => TraitItem::Type(TraitItemType {
                attrs: type_.attrs,
                type_token: type_.type_token,
                ident: type_.ident,
                generics: type_.generics,
                colon_token: None,
                bounds: Punctuated::new(),
                default: None,
                semi_token: type_.semi_token,
            }),
            ImplItem::Macro(macro_) => TraitItem::Macro(TraitItemMacro {
                attrs: macro_.attrs,
                mac: macro_.mac,
                semi_token: macro_.semi_token,
            }),
            ImplItem::Verbatim(token_stream) => {
                TraitItem::Verbatim(token_stream)
            }
            u => {
                return Error::new(
                    Span::call_site(),
                    format!("Unknown `ImplItem` variant: {u:?}"),
                )
                .into_compile_error()
                .into()
            }
        };

        item.to_tokens(&mut items);
    }

    quote::quote! {
        #block

        #[::traitful::seal(for #generics #self_ty)]
        #attrs
        #vis #unsafety trait #ident {
            #items
        }
    }
    .into()
}
