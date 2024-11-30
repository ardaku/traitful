use syn::{GenericParam, Generics, Ident};

/// Obfuscate user-defined generics, so they don't conflict with traitful's
/// additional generics.
pub(crate) fn obfuscate_generics(mut generics: Generics) -> Generics {
    for param in generics.params.iter_mut() {
        match param {
            GenericParam::Lifetime(ref mut param) => {
                obfuscate_ident(&mut param.lifetime.ident);
            }
            GenericParam::Type(ref mut param) => {
                obfuscate_ident(&mut param.ident);
            }
            GenericParam::Const(ref mut param) => {
                obfuscate_ident(&mut param.ident);
            }
        }
    }

    generics
}

/// Obfuscate an ident by adding a trailing '_'.
fn obfuscate_ident(ident: &mut Ident) {
    *ident = Ident::new(&format!("{ident}_"), ident.span())
}
