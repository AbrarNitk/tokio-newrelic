use crate::tables::{users_skill::dsl, users_skill::dsl::users_skill};

use tokio_newrelic;

use diesel::prelude::*;
use tokio_newrelic::pg::NConnection;

#[derive(Queryable, Debug)]
pub struct Skill {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub allocation_logic: String,
}

fn query(conn: &NConnection) {
    let nr_result: Vec<Skill> = users_skill
        .filter(dsl::id.gt(20))
        .load::<Skill>(conn)
        .expect("Error loading skills");
    println!("{:?}", nr_result.len());
}

pub fn db_test() {
    println!("pg_db_test");
    let database_url = "postgres://root@127.0.0.1/acko";
    let nr_conn = tokio_newrelic::pg::NConnection::establish(database_url)
        .expect(&format!("Error connecting to {}", database_url));
    query(&nr_conn);
}

pub fn db_test_pooled_connection() {
    println!("pg_db_test_pooled_connection");
    let database_url = "postgres://root@127.0.0.1/acko";
    let pooled_conn = tokio_newrelic::pg_pool::connection_with_url(database_url);
    query(&pooled_conn);
}
