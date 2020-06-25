#[macro_use]
extern crate tokio;
//#[macro_use]
//extern crate tokio_db_newrelic;

use std::cell::Cell;

tokio::task_local! {
    static NUMBER: u32;
}

// task_local!(static REQUEST_ID: Cell<u64> = Cell::new(0));

//#[temp_newrelic]
async fn abc1() {
    println!("abc1 {}", NUMBER.get());
    let t = NUMBER.try_with(|value| {
        println!("valueeee: {}", value);
        *value
    });

    println!("{:#?}", t);

    NUMBER.inner.with(|value| {
        println!("Option valueeee: {:?}", value);
    });
}

fn value(u: &u32) -> u32 {
    println!("valueeee: {}", u);
    *u
}

pub async fn abc() -> i32 {
    let t1 = NUMBER.try_with(&value);
    println!("{:#?}", t1);

    NUMBER.inner.with(|value| {
        println!("Option valueeee1: {:?}", value);
    });

    let t: i32 = NUMBER
        .scope(1, async move {
            println!("abc {}", NUMBER.get());
            abc1().await;
            return 2;
        })
        .await;
    t
}
