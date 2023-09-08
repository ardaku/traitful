use proc_macro2::Span;
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    token::{For, Gt, Lt},
    GenericParam, Generics, Result, Token, Type,
};

#[derive(Clone)]
pub(crate) struct BoundGenerics {
    _for_token: Token![for],
    lt_token: Token![<],
    pub(crate) params: Punctuated<GenericParam, Token![,]>,
    gt_token: Token![>],
}

impl Parse for BoundGenerics {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        Ok(Self {
            _for_token: input.parse()?,
            lt_token: input.parse()?,
            params: {
                let mut generics = Punctuated::new();

                while !input.peek(Token![>]) {
                    let generic: GenericParam = input.parse()?;

                    generics.push_value(generic);

                    if input.peek(Token![>]) {
                        break;
                    }

                    generics.push_punct(input.parse()?);
                }

                generics
            },
            gt_token: input.parse()?,
        })
    }
}

impl From<Punctuated<GenericParam, Token![,]>> for BoundGenerics {
    fn from(params: Punctuated<GenericParam, Token![,]>) -> Self {
        Self {
            _for_token: For {
                span: Span::call_site(),
            },
            lt_token: Lt {
                spans: [Span::call_site()],
            },
            params,
            gt_token: Gt {
                spans: [Span::call_site()],
            },
        }
    }
}

impl From<BoundGenerics> for Generics {
    fn from(bound_generics: BoundGenerics) -> Self {
        Self {
            lt_token: Some(bound_generics.lt_token),
            params: bound_generics.params,
            gt_token: Some(bound_generics.gt_token),
            where_clause: None,
        }
    }
}

pub(crate) struct BoundGenericsType {
    pub(crate) bound_generics: Option<BoundGenerics>,
    pub(crate) type_: Type,
}

impl Parse for BoundGenericsType {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        Ok(Self {
            bound_generics: if input.lookahead1().peek(Token![for]) {
                Some(BoundGenerics::parse(input)?)
            } else {
                None
            },
            type_: if input.lookahead1().peek(Token![for]) {
                let for_: For = input.parse()?;

                return Err(syn::Error::new(for_.span, "unexpected `for`"));
            } else {
                input.parse()?
            },
        })
    }
}
