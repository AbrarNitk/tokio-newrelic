#[macro_use]
extern crate lazy_static;

use std::env;
use std::future::Future;

use newrelic::{self, App};
use std::str::FromStr;

pub mod newrelic_fns;
pub mod pg;
pub(crate) mod sql_parser;

tokio::task_local! {
    pub static TL_TRANSACTION: Option<newrelic::Transaction>;
}

fn create_transaction(name: &str) -> Option<newrelic::Transaction> {
    if *crate::ENABLE_NEW_RELIC {
        match NR_APP.web_transaction(name) {
            Ok(trans) => Some(trans),
            Err(e) => {
                println!("Error init web transaction {} :: {:?}", name, e);
                None
            }
        }
    } else {
        println!("Newrelic is not enabled for starting a web transaction");
        None
    }
}

fn init_nr_app() -> App {
    let license_key = env::var("NEW_RELIC_LICENSE_KEY").unwrap_or_else(|_| "".to_string());
    let app_name = env::var("NEW_RELIC_APP_NAME").unwrap_or_else(|_| "acko_api_test".to_string());
    let app = App::new(&app_name, &license_key).expect("Could not create app");
    app
}

fn enable_nr_app() -> bool {
    let enable_nr = env::var("ENABLE_NEW_RELIC").unwrap_or_else(|_| "false".to_string());
    let x: bool = FromStr::from_str(&enable_nr).unwrap();
    x
}

lazy_static! {
    pub static ref NR_APP: App = init_nr_app();
    pub static ref ENABLE_NEW_RELIC: bool = enable_nr_app();
}

pub async fn execute<F>(transaction_name: &str, f: F) -> F::Output
where
    F: Future,
{
    TL_TRANSACTION
        .scope(create_transaction(transaction_name), f)
        .await
}

pub async fn abc1() {
    TL_TRANSACTION.inner.with(|value| {
        match value.borrow().as_ref() {
            Some(tr) => {
                println!("TL Option valueeee: {:#?}", tr.is_some());
            }
            None => {}
        };
        // println!("TL Option valueeee: {:#?}", value.borrow().is_some());
    });

    // crate::newrelic_fns::start_custom_segment("abc1");
}
