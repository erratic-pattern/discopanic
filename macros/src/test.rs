use proc_macro2::Span;
use quote::quote;
use syn::{parse_quote_spanned, spanned::Spanned, token::Bracket, Attribute, ItemFn, Meta, Token};

pub(crate) fn gen_test(attr_meta: Meta, test_fn: ItemFn) -> proc_macro2::TokenStream {
    // add #[test] if missing
    let ItemFn {
        attrs,
        vis,
        sig,
        block,
    } = normalize_fn_test_attr(attr_meta.span(), test_fn);
    quote! {
        #(#attrs)*
        #vis #sig
        {
            let _orig_hook = std::panic::take_hook();
            discopanic::install();
            let ret = {
                #block
            };
            std::panic::set_hook(_orig_hook);
            ret
        }
    }
}

/// Modifies a function definition to include a `#[test]` attribute macro if none is currently
/// provided.
pub(crate) fn normalize_fn_test_attr(span: Span, mut item_fn: ItemFn) -> ItemFn {
    if fn_has_test_attr(&item_fn) {
        item_fn
    } else {
        item_fn.attrs.push(gen_test_attr(span));
        item_fn
    }
}

/// check if an ItemFn has a `#[test]` attribute macro
pub(crate) fn fn_has_test_attr(ItemFn { attrs, .. }: &ItemFn) -> bool {
    attrs
        .iter()
        .find(|&attr| attr.path().is_ident("test"))
        .is_none()
}

/// Generate a `#[test]` attribute in the given Span
pub(crate) fn gen_test_attr(span: Span) -> Attribute {
    Attribute {
        pound_token: Token![#]([span]),
        style: syn::AttrStyle::Outer,
        bracket_token: Bracket(span),
        meta: parse_quote_spanned!(span => test),
    }
}
