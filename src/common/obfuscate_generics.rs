use syn::{
    punctuated::Punctuated, token::Comma, CapturedParam, GenericParam,
    Generics, Ident, TypeParamBound,
};

/// Obfuscate user-defined generics, so they don't conflict with traitful's
/// additional generics.
pub(crate) fn obfuscate_generics(mut generics: Generics) -> Generics {
    obfuscate_params(&mut generics.params);
    generics
}

/// Obfuscate the generic parameters.
fn obfuscate_params(params: &mut Punctuated<GenericParam, Comma>) {
    for param in params.iter_mut() {
        match param {
            GenericParam::Lifetime(ref mut param) => {
                obfuscate_ident(&mut param.lifetime.ident);
            }
            GenericParam::Type(ref mut param) => {
                obfuscate_ident(&mut param.ident);

                for bound in param.bounds.iter_mut() {
                    match bound {
                        TypeParamBound::Trait(ref mut bound) => {
                            if let Some(ref mut lifetimes) = bound.lifetimes {
                                obfuscate_params(&mut lifetimes.lifetimes);
                            }
                        }
                        TypeParamBound::Lifetime(ref mut bound) => {
                            obfuscate_ident(&mut bound.ident);
                        }
                        TypeParamBound::PreciseCapture(ref mut bound) => {
                            for param in bound.params.iter_mut() {
                                match param {
                                    CapturedParam::Lifetime(ref mut param) => {
                                        obfuscate_ident(&mut param.ident);
                                    }
                                    CapturedParam::Ident(ref mut param) => {
                                        obfuscate_ident(param);
                                    }
                                    _ => {}
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
            GenericParam::Const(ref mut param) => {
                obfuscate_ident(&mut param.ident);
            }
        }
    }
}

/// Obfuscate an ident by adding a trailing '_'.
fn obfuscate_ident(ident: &mut Ident) {
    *ident = Ident::new(&format!("{ident}_"), ident.span())
}
