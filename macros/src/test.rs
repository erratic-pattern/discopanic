use quote::quote;
use syn::ItemFn;

pub(crate) fn gen_test(test_fn: ItemFn) -> proc_macro2::TokenStream {
    let ItemFn {
        attrs,
        vis,
        sig,
        block,
    } = test_fn;
    // add #[test] if missing
    let test_attr = if attrs
        .iter()
        .find(|&attr| attr.path().is_ident("test"))
        .is_none()
    {
        Some(quote! {#[test]})
    } else {
        None
    };
    quote! {
        #test_attr
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
