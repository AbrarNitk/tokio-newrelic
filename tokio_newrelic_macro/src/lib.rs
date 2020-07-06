#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;

#[proc_macro_attribute]
pub fn newrelic_transaction(
    _metadata: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let input_fn: syn::ItemFn = parse_macro_input!(input as syn::ItemFn);
    let visibility = input_fn.vis;
    let asyncness = input_fn.sig.asyncness;
    let generics = &input_fn.sig.generics;
    let ident = &input_fn.sig.ident;
    let ident_name = ident.to_string();
    let inputs = input_fn.sig.inputs;
    let output = input_fn.sig.output;
    let where_clause = &input_fn.sig.generics.where_clause;
    let block = input_fn.block;
    (quote!(
        #visibility #asyncness fn #ident #generics (#inputs) #output #where_clause {
            let f = move || async move { #block };
            let r = tokio_newrelic::execute(#ident_name, async move {
                f().await
            }).await;
            r
        }
    ))
    .into()
}
