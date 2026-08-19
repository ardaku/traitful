use syn::{
    CapturedParam, GenericParam, Generics, Ident, TypeParamBound,
    punctuated::Punctuated, token::Comma,
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
            GenericParam::Lifetime(param) => {
                obfuscate_ident(&mut param.lifetime.ident);
            }
            GenericParam::Type(param) => {
                obfuscate_ident(&mut param.ident);

                for bound in param.bounds.iter_mut() {
                    match bound {
                        TypeParamBound::Trait(bound) => {
                            if let Some(ref mut lifetimes) = bound.lifetimes {
                                obfuscate_params(&mut lifetimes.lifetimes);
                            }
                        }
                        TypeParamBound::Lifetime(bound) => {
                            obfuscate_ident(&mut bound.ident);
                        }
                        TypeParamBound::PreciseCapture(bound) => {
                            for param in bound.params.iter_mut() {
                                match param {
                                    CapturedParam::Lifetime(param) => {
                                        obfuscate_ident(&mut param.ident);
                                    }
                                    CapturedParam::Ident(param) => {
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
            GenericParam::Const(param) => {
                obfuscate_ident(&mut param.ident);
            }
        }
    }
}

/// Obfuscate an ident by adding a trailing '_'.
fn obfuscate_ident(ident: &mut Ident) {
    *ident = Ident::new(&format!("{ident}_"), ident.span())
}
