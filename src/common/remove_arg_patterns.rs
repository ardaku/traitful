use syn::{FnArg, Pat, Signature};

/// Remove `mut` references from function arguments in signature.
pub(crate) fn remove_arg_patterns(signature: &mut Signature) {
    for arg in signature.inputs.iter_mut() {
        match arg {
            FnArg::Receiver(ref mut receiver) => {
                if receiver.reference.is_none() {
                    receiver.mutability = None;
                }
            }
            FnArg::Typed(ref mut typed) => remove_pat_patterns(&mut typed.pat),
        }
    }
}

fn remove_pat_patterns(typed: &mut Pat) {
    match typed {
        Pat::Ident(ident) => {
            if ident.by_ref.is_none() {
                ident.mutability = None;
            }
        }
        Pat::Or(or) => {
            for pat in or.cases.iter_mut() {
                remove_pat_patterns(pat);
            }
        }
        Pat::Paren(paren) => remove_pat_patterns(&mut paren.pat),
        Pat::Reference(reference) => remove_pat_patterns(&mut reference.pat),
        Pat::Slice(slice) => {
            for pat in slice.elems.iter_mut() {
                remove_pat_patterns(pat);
            }
        }
        Pat::Struct(tuple) => {
            for pat in tuple.fields.iter_mut() {
                remove_pat_patterns(&mut pat.pat);
            }
        }
        Pat::Tuple(tuple) => {
            for pat in tuple.elems.iter_mut() {
                remove_pat_patterns(pat);
            }
        }
        Pat::TupleStruct(tuple) => {
            for pat in tuple.elems.iter_mut() {
                remove_pat_patterns(pat);
            }
        }
        Pat::Type(ty) => remove_pat_patterns(&mut ty.pat),
        _ => {}
    }
}
