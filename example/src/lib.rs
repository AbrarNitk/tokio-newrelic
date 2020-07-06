#[macro_use]
extern crate tokio_newrelic_macro;
#[macro_use]
extern crate diesel;

use tokio_newrelic;

mod db_test;
mod tables;

pub async fn abc1() {
    println!("call async function");
}

#[newrelic_transaction]
pub async fn newrelic_transaction_function() -> Option<i32> {
    self::abc1().await;
    db_test::db_test();
    db_test::db_test_pooled_connection();
    std::thread::sleep(std::time::Duration::from_secs(2));
    Some(2)
}

pub async fn newrelic_transaction_function1() -> Option<i32> {
    let r = tokio_newrelic::execute("web_transaction_name", async move {
        self::abc1().await;
        db_test::db_test();
        db_test::db_test_pooled_connection();
        std::thread::sleep(std::time::Duration::from_secs(2));
        Some(2)
    })
    .await;
    r
}
