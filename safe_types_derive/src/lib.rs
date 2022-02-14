use proc_macro::{token_stream::IntoIter, TokenStream, TokenTree};
use quote::quote;
use syn::parse::Parser;
use syn::{parse_quote, punctuated::Punctuated, FnArg, ItemFn, Signature, Token};

/// Implements methods
#[proc_macro]
pub fn impl_methods(input: TokenStream) -> TokenStream {
    let mut token_iter = input.into_iter();

    let owned = get_ident(&mut token_iter);
    get_punct(&mut token_iter);
    let by_ref = get_ident(&mut token_iter);
    get_punct(&mut token_iter);
    let by_ref_mut = get_ident(&mut token_iter);
    get_punct(&mut token_iter);

    let signatures = if let TokenTree::Group(grp) = token_iter.next().unwrap() {
        grp.stream()
    } else {
        panic!()
    };

    let parser = Punctuated::<Signature, Token![;]>::parse_terminated;
    let signatures = parser.parse(signatures).unwrap();

    let mut implementations = Vec::<ItemFn>::new();

    for signature in signatures {
        let receiver = if let FnArg::Receiver(rcv) = signature
            .inputs
            .first()
            .expect("methods must have `self` receiver")
        {
            rcv
        } else {
            panic!("methods must have `self` receiver")
        };

        let conversion_method = match (&receiver.reference, &receiver.mutability) {
            (None, None) => &owned,
            (Some(_), None) => &by_ref,
            _ => &by_ref_mut,
        };

        let fn_name = signature.ident.clone();

        let args = signature
            .inputs
            .clone()
            .into_iter()
            .skip(1)
            .map(|arg| match arg {
                FnArg::Typed(pat) => pat.pat,
                _ => panic!(),
            });

        implementations.push(ItemFn {
            attrs: Vec::new(),
            vis: parse_quote! { pub },
            sig: signature,
            block: Box::new(parse_quote! {{
                self. #conversion_method () . #fn_name ( #( #args ),*)
            }}),
        });
    }

    quote! {
        #( #implementations )*
    }
    .into()
}

fn get_ident(token_iter: &mut IntoIter) -> proc_macro2::Ident {
    if let TokenTree::Ident(ident) = token_iter.next().unwrap() {
        proc_macro2::Ident::new(&format!("{}", ident), proc_macro2::Span::call_site())
    } else {
        panic!()
    }
}

fn get_punct(token_iter: &mut IntoIter) {
    if let TokenTree::Punct(_) = token_iter.next().unwrap() {
    } else {
        panic!()
    }
}
