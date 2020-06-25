#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;
#[macro_use]
extern crate tokio;

use std::cell::Cell;

tokio::task_local! {
    static NUMBER: u32;
}

#[proc_macro_attribute]
pub fn temp_newrelic(
    metadata: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let input_fn: syn::ItemFn = parse_macro_input!(input as syn::ItemFn);
    let visibility = input_fn.vis;
    let asyncness = input_fn.sig.asyncness;
    let generics = &input_fn.sig.generics;
    let ident = input_fn.sig.ident;
    let inputs = input_fn.sig.inputs;
    let output = input_fn.sig.output;
    let where_clause = &input_fn.sig.generics.where_clause;
    let block = input_fn.block;

    // proc_macro::TokenStream::from(input).into()
    (quote!(
        #visibility #asyncness fn #ident #generics (#inputs) #output #where_clause {
            println!("macro is working fine");
            let f = || { #block };
            let r = f();
            r
        }
    ))
    .into()
}

// task_local!(static REQUEST_ID: Cell<u64> = Cell::new(0));

async fn abc() -> i32 {
    println!("value {}", NUMBER.get());
    let t = NUMBER
        .scope(1, async move {
            println!("value {}", NUMBER.get());
            return 2;
        })
        .await;
    t
}

fn abc1() {
    println!("value1 {}", NUMBER.get());
}

fn abc2() {
    println!("value2 {}", NUMBER.get());
}

#[cfg(test)]
mod tests {
    use tokio::runtime::Runtime;

    use super::abc1;

    #[test]
    fn it_works() {
        let mut rt = Runtime::new().unwrap();
        rt.block_on(async {
            //            super::abc1();
            //            super::abc2();
            super::NUMBER
                .scope(1, async move {
                    let t = super::abc().await;
                    println!("Value of t: {}", t);
                    super::abc1();
                    super::abc2();
                })
                .await;
        });

        // std::thread::sleep(std::time::Duration::from_secs(1));
        assert_eq!(2 + 2, 4);
    }
}
