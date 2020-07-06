## New Relic SDK for async tasks(tokio and actix)
It is an wrapper over the newrelic SDK.

## Usage

```toml
tokio-newrelic = "*"
```

#### Export variables
```shell script
export ENABLE_NEW_RELIC="true"
export NEW_RELIC_LICENSE_KEY="newrelic_license_key"
export NEW_RELIC_APP_NAME="app_name"
```


```rust
use tokio_db_newrelic;

// actix api function
#[get("/")]
async fn index(_req: HttpRequest) -> impl Responder {
    let t = newrelic_transaction_function().await;
    HttpResponse::Ok().body(format!("index_page {:?}", t))
}

pub async fn newrelic_transaction_function1() -> Option<i32> {
    // Mandatory wrapping, with newrelic for setting task scope 
    // starting a web transaction and storing it to a tokio::Localtask 
    let r = tokio_db_newrelic::execute("web_transaction_name", async move {
        self::abc1().await;
        db_test();
        db_test_pooled_connection();
        std::thread::sleep(std::time::Duration::from_secs(2));
        Some(2)
    })
    .await;
    r
}

pub fn db_test_pooled_connection() {
    println!("pg_db_test_pooled_connection");
    let database_url = "postgres://root@127.0.0.1/acko";
    let pooled_conn = tokio_db_newrelic::pg_pool::connection_with_url(database_url);
    query(&pooled_conn);
}


pub fn db_test() {
    println!("pg_db_test");
    let database_url = "postgres://root@127.0.0.1/acko";
    let nr_conn = tokio_db_newrelic::pg::NConnection::establish(database_url)
        .expect(&format!("Error connecting to {}", database_url));
    query(&nr_conn);
}
```

#### Testing 
```shell script
 for ((i=1;i<=100;i++)); do seq 1 200 | xargs -n2 -P20  curl "http://127.0.0.1:3000/"; done
```

- Hope that you got an idea, see the example directory for more information.
- This crate is implemented only for datastore segment.

* [ ] Segments 
    * [x] Datastore
    * [ ] Custom
    * [ ] External
    * [ ] Nesting Segments
    * [ ] Overriding timings
    

## Need to run c-sdk daemon
This crate requires the newrelic daemon to running as per the docs [Newrelic docs][c-sdk];
 
[c-sdk]: https://docs.newrelic.com/docs/agents/c-sdk/get-started/introduction-c-sdk#architecture