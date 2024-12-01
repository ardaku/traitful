use syn::{
    punctuated::Punctuated,
    token::{Comma, Plus},
    CapturedParam, Expr, GenericArgument, GenericParam, Ident, Lifetime, Path,
    PathArguments, ReturnType, Type, TypeParamBound,
};

pub(crate) fn obfuscate_supertraits(
    supertraits: &Punctuated<TypeParamBound, Plus>,
    unobfuscated_generics: &Punctuated<GenericParam, Comma>,
    obfuscated_generics: &Punctuated<GenericParam, Comma>,
) -> Punctuated<TypeParamBound, Plus> {
    supertraits
        .iter()
        .cloned()
        .map(|mut bound| {
            match bound {
                TypeParamBound::Trait(ref mut bound) => obfuscate_path(
                    &mut bound.path,
                    unobfuscated_generics,
                    obfuscated_generics,
                ),
                TypeParamBound::Lifetime(ref mut lifetime) => {
                    obfuscate_lifetime(
                        lifetime,
                        unobfuscated_generics,
                        obfuscated_generics,
                    )
                }
                TypeParamBound::PreciseCapture(ref mut pc) => {
                    for param in pc.params.iter_mut() {
                        match param {
                            CapturedParam::Lifetime(ref mut lifetime) => {
                                obfuscate_lifetime(
                                    lifetime,
                                    unobfuscated_generics,
                                    obfuscated_generics,
                                )
                            }
                            CapturedParam::Ident(ref mut ident) => {
                                obfuscate_ident(
                                    ident,
                                    unobfuscated_generics,
                                    obfuscated_generics,
                                )
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }

            bound
        })
        .collect()
}

/// An ident could be a a generic, check it.
fn obfuscate_ident(
    ident: &mut Ident,
    unobfuscated_generics: &Punctuated<GenericParam, Comma>,
    obfuscated_generics: &Punctuated<GenericParam, Comma>,
) {
    for (unobfuscated, obfuscated) in
        unobfuscated_generics.iter().zip(obfuscated_generics)
    {
        let GenericParam::Type(unobfuscated_param) = unobfuscated else {
            continue;
        };
        let GenericParam::Type(obfuscated_param) = obfuscated else {
            continue;
        };

        if *ident == unobfuscated_param.ident {
            *ident = obfuscated_param.ident.clone();
        }
    }
}

/// An lifetime should be a a generic, check it.
fn obfuscate_lifetime(
    lifetime: &mut Lifetime,
    unobfuscated_generics: &Punctuated<GenericParam, Comma>,
    obfuscated_generics: &Punctuated<GenericParam, Comma>,
) {
    for (unobfuscated, obfuscated) in
        unobfuscated_generics.iter().zip(obfuscated_generics)
    {
        let GenericParam::Lifetime(unobfuscated_lifetime) = unobfuscated else {
            continue;
        };
        let GenericParam::Lifetime(obfuscated_lifetime) = obfuscated else {
            continue;
        };

        if lifetime.ident == unobfuscated_lifetime.lifetime.ident {
            lifetime.ident = obfuscated_lifetime.lifetime.ident.clone();
        }
    }
}

fn obfuscate_generic(
    generic: &mut GenericArgument,
    unobfuscated_generics: &Punctuated<GenericParam, Comma>,
    obfuscated_generics: &Punctuated<GenericParam, Comma>,
) {
    match generic {
        GenericArgument::Lifetime(ref mut arg) => {
            obfuscate_lifetime(arg, unobfuscated_generics, obfuscated_generics)
        }
        GenericArgument::Type(ref mut arg) => {
            obfuscate_type(arg, unobfuscated_generics, obfuscated_generics);
        }
        GenericArgument::Const(ref mut expr) => {
            // FIXME
            match expr {
                Expr::Const(_) => unimplemented!("const expr unsupported"),
                Expr::Array(_) => unimplemented!("const array unsupported"),
                Expr::Paren(_) => unimplemented!("const paren unsupported"),
                Expr::Path(ref mut path) => {
                    obfuscate_path(
                        &mut path.path,
                        unobfuscated_generics,
                        obfuscated_generics,
                    );
                }
                _ => {}
            }
        }
        _ => {}
    }
}

fn obfuscate_path(
    path: &mut Path,
    unobfuscated_generics: &Punctuated<GenericParam, Comma>,
    obfuscated_generics: &Punctuated<GenericParam, Comma>,
) {
    for segment in path.segments.iter_mut() {
        match segment.arguments {
            PathArguments::None => {}
            PathArguments::AngleBracketed(ref mut args) => {
                for arg in args.args.iter_mut() {
                    obfuscate_generic(
                        arg,
                        unobfuscated_generics,
                        obfuscated_generics,
                    );
                }
            }
            PathArguments::Parenthesized(ref mut args) => {
                for arg in args.inputs.iter_mut() {
                    obfuscate_type(
                        arg,
                        unobfuscated_generics,
                        obfuscated_generics,
                    );
                }

                if let ReturnType::Type(_, ref mut ty) = args.output {
                    obfuscate_type(
                        ty,
                        unobfuscated_generics,
                        obfuscated_generics,
                    );
                }
            }
        }
    }
}

fn obfuscate_type(
    ty: &mut Type,
    unobfuscated_generics: &Punctuated<GenericParam, Comma>,
    obfuscated_generics: &Punctuated<GenericParam, Comma>,
) {
    match ty {
        Type::Array(ref mut inner) => obfuscate_type(
            &mut inner.elem,
            unobfuscated_generics,
            obfuscated_generics,
        ),
        Type::Slice(ref mut inner) => obfuscate_type(
            &mut inner.elem,
            unobfuscated_generics,
            obfuscated_generics,
        ),
        Type::BareFn(ref mut bare_fn) => {
            for arg in bare_fn.inputs.iter_mut() {
                obfuscate_type(
                    &mut arg.ty,
                    unobfuscated_generics,
                    obfuscated_generics,
                );
            }

            if let ReturnType::Type(_, ref mut ty) = bare_fn.output {
                obfuscate_type(ty, unobfuscated_generics, obfuscated_generics);
            }
        }
        Type::Group(ref mut group) => {
            obfuscate_type(
                &mut group.elem,
                unobfuscated_generics,
                obfuscated_generics,
            );
        }
        Type::ImplTrait(ref mut impl_trait) => {
            impl_trait.bounds = obfuscate_supertraits(
                &impl_trait.bounds,
                unobfuscated_generics,
                obfuscated_generics,
            );
        }
        Type::Paren(ref mut paren) => {
            obfuscate_type(
                &mut paren.elem,
                unobfuscated_generics,
                obfuscated_generics,
            );
        }
        Type::Ptr(ref mut ptr) => {
            obfuscate_type(
                &mut ptr.elem,
                unobfuscated_generics,
                obfuscated_generics,
            );
        }
        Type::Reference(ref mut reference) => {
            if let Some(ref mut lifetime) = reference.lifetime {
                obfuscate_lifetime(
                    lifetime,
                    unobfuscated_generics,
                    obfuscated_generics,
                );
            }

            obfuscate_type(
                &mut reference.elem,
                unobfuscated_generics,
                obfuscated_generics,
            );
        }
        Type::Tuple(ref mut tuple) => {
            for elem in tuple.elems.iter_mut() {
                obfuscate_type(
                    elem,
                    unobfuscated_generics,
                    obfuscated_generics,
                );
            }
        }
        Type::TraitObject(ref mut object) => {
            object.bounds = obfuscate_supertraits(
                &object.bounds,
                unobfuscated_generics,
                obfuscated_generics,
            );
        }
        Type::Path(ref mut path) => {
            obfuscate_path(
                &mut path.path,
                unobfuscated_generics,
                obfuscated_generics,
            );
        }
        _ => {}
    }
}
