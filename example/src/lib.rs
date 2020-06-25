#[macro_use]
extern crate tokio;

#[macro_use]
extern crate tokio_newrelic_macro;

use tokio_db_newrelic;

use std::cell::Cell;

#[temp_newrelic]
pub async fn abc() -> Option<i32> {
    tokio_db_newrelic::abc1().await;
    std::thread::sleep(std::time::Duration::from_secs(2));
    Some(2)
}
