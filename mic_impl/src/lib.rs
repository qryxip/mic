//! Implementation for [`mic`].
//!
//! This crate is not intended to be used directly.
//! Use via [`mic`] crate.
//!
//! [`mic`]: https://docs.rs/crate/mic
#![forbid(unsafe_code)]
#![warn(
    bare_trait_objects,
    ellipsis_inclusive_range_patterns,
    elided_lifetimes_in_paths,
    explicit_outlives_requirements
)]

extern crate proc_macro;

use itertools::Itertools as _;
use quote::{quote, ToTokens as _};
use syn::{
    parse::{Parse, ParseStream},
    parse_quote,
    punctuated::Punctuated,
    spanned::Spanned as _,
    Expr, FnArg, ItemFn, Pat, PatIdent, PatType, Signature, Token,
};

/// Wraps the output in [`println!`].
///
/// See the crate level documentation for details.
///
/// [`println!`]: https://doc.rust-lang.org/nightly/std/macro.println.html
#[proc_macro_attribute]
pub fn answer(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    run(attr.into(), item.into(), Kind::Answer)
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}

/// Wraps the output in [`ToString::to_string`].
///
/// See the crate level documentation for details.
///
/// [`ToString::to_string`]: https://doc.rust-lang.org/nightly/std/string/trait.ToString.html#tymethod.to_string
#[proc_macro_attribute]
pub fn solve(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    run(attr.into(), item.into(), Kind::Solve)
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}

fn run(
    attr: proc_macro2::TokenStream,
    item: proc_macro2::TokenStream,
    kind: Kind,
) -> syn::Result<proc_macro2::TokenStream> {
    let Exprs(convertions) = syn::parse2(attr)?;

    let item = &syn::parse2::<ItemFn>(item)?;
    let ItemFn { sig, block, .. } = item;
    let Signature {
        constness,
        asyncness,
        unsafety,
        abi,
        fn_token,
        ident,
        generics,
        inputs,
        variadic,
        output,
        ..
    } = sig;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    if let Some(constness) = constness {
        return Err(syn::Error::new(constness.span(), "`const` not allowed"));
    }
    if let Some(variadic) = variadic {
        return Err(syn::Error::new(variadic.span(), "`...` not allowed"));
    }

    let final_output_ty = kind.final_output_ty();

    let args = to_args(&sig.inputs)?;

    let convertion = convertions
        .into_iter()
        .map(|f| f.to_token_stream())
        .rev()
        .fold1(|g, f| quote!(dot(#f, #g)));

    let converted = match convertion {
        Some(convertion) => quote!(#convertion(__mic_ans)),
        None => quote!(__mic_ans),
    };
    let converted = quote!({
        #[allow(unused_imports)]
        use ::mic::filters::*;
        #converted
    });

    if *output == parse_quote!(-> _) {
        let stmt = match kind {
            Kind::Answer => quote!(::std::println!("{}", __mic_ans);),
            Kind::Solve => quote!(::std::string::ToString::to_string(&__mic_ans)),
        };

        return Ok(quote! {
            #asyncness #unsafety #abi #fn_token #ident#impl_generics(#inputs) #final_output_ty
                #where_clause
            {
                #[allow(unused_imports)]
                use ::mic::__YouCannotRecurseIfTheOutputTypeIsInferred as #ident;

                let __mic_ans = (move || -> _ #block)();
                let __mic_ans = #converted;

                #stmt
            }
        });
    } else {
        let turbofish = ty_generics.as_turbofish();
        let stmt = match kind {
            Kind::Answer => quote!(::std::println!("{}", __mic_ans);),
            Kind::Solve => quote!(return ::std::string::ToString::to_string(&__mic_ans);),
        };

        return Ok(quote! {
            #asyncness #unsafety #abi #fn_token #ident#impl_generics(#inputs) #final_output_ty
                #where_clause
            {
                let __mic_ans = #ident#turbofish(#args);
                let __mic_ans = #converted;

                #stmt

                #item
            }
        });
    }

    struct Exprs(Punctuated<Expr, Token![,]>);

    impl Parse for Exprs {
        fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
            Punctuated::parse_terminated(input).map(Self)
        }
    }
}

fn to_args(inputs: &Punctuated<FnArg, Token![,]>) -> syn::Result<proc_macro2::TokenStream> {
    inputs
        .iter()
        .map(|arg| match arg {
            FnArg::Receiver(_) => Ok(quote!(self)),
            FnArg::Typed(PatType { pat, .. }) => match &**pat {
                Pat::Ident(PatIdent { ident, .. }) => Ok(quote!(#ident)),
                pat => Err(syn::Error::new(pat.span(), "unsupported argument pattern")),
            },
        })
        .collect()
}

#[derive(Copy, Clone)]
enum Kind {
    Answer,
    Solve,
}

impl Kind {
    fn final_output_ty(self) -> proc_macro2::TokenStream {
        match self {
            Kind::Answer => quote!(),
            Kind::Solve => quote!(-> ::std::string::String),
        }
    }
}
