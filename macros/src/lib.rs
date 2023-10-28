mod test;
use syn::ItemFn;
use test::gen_test;

#[proc_macro_attribute]
pub fn test(_: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let test_fn = syn::parse::<ItemFn>(item).expect("Could not parse integration_test function");
    gen_test(test_fn).into()
}
