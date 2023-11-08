mod test;
use syn::{ItemFn, Meta};
use test::gen_test;

#[proc_macro_attribute]
pub fn test(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let attr_meta = syn::parse::<Meta>(attr).expect("Could not parse test attribute");
    let test_fn = syn::parse::<ItemFn>(item).expect("Could not parse test function");
    gen_test(attr_meta, test_fn).into()
}
