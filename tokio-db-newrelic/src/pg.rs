use diesel::prelude::*;
use diesel::query_builder::QueryBuilder;

use crate::ENABLE_NEW_RELIC;

pub struct DebugConnection {
    pub conn: diesel::PgConnection,
}

pub type NConnection = DebugConnection;

impl diesel::connection::SimpleConnection for NConnection {
    fn batch_execute(&self, query: &str) -> QueryResult<()> {
        self.conn.batch_execute(query)
    }
}

impl NConnection {
    pub fn new(url: &str) -> diesel::result::ConnectionResult<Self> {
        Ok(NConnection {
            conn: diesel::PgConnection::establish(url)?,
        })
    }

    pub fn build_transaction(&self) -> diesel::pg::TransactionBuilder {
        self.conn.build_transaction()
    }
}

fn execute_fn<U, F: FnOnce() -> U>(table: &str, operation: &str, query: &str, f: F) -> U {
    let segment_params = newrelic::DatastoreParamsBuilder::new(newrelic::Datastore::Postgres)
        .collection(&table)
        .operation(&operation)
        .query(&query.replace("\"", ""))
        .build()
        .expect("Invalid data store parameters");

    crate::TL_TRANSACTION.inner.with(|value| {
        let t = value.borrow();
        if let Some(v) = t.as_ref() {
            if let Some(trans) = v.as_ref() {
                trans.datastore_segment(&segment_params, |_| f())
            } else {
                f()
            }
        } else {
            f()
        }
    })
}

impl diesel::connection::Connection for NConnection {
    type Backend = diesel::pg::Pg;
    type TransactionManager = diesel::connection::AnsiTransactionManager;

    fn establish(url: &str) -> ConnectionResult<Self> {
        let f = || NConnection::new(url);
        if *ENABLE_NEW_RELIC {
            execute_fn("connection", "establish_connection", "", f)
        } else {
            f()
        }
    }

    fn execute(&self, query: &str) -> QueryResult<usize> {
        let f = || self.conn.execute(query);
        if *ENABLE_NEW_RELIC {
            let (operation, table) = crate::sql_parser::parse_sql(&query);
            execute_fn(&table, &operation, query, f)
        } else {
            f()
        }
    }

    fn query_by_index<T, U>(&self, source: T) -> QueryResult<Vec<U>>
    where
        T: diesel::query_builder::AsQuery,
        T::Query:
            diesel::query_builder::QueryFragment<diesel::pg::Pg> + diesel::query_builder::QueryId,
        diesel::pg::Pg: diesel::sql_types::HasSqlType<T::SqlType>,
        U: diesel::deserialize::Queryable<T::SqlType, diesel::pg::Pg>,
    {
        let query = source.as_query();
        let debug_query = diesel::debug_query(&query).to_string();
        let f = || self.conn.query_by_index(query);
        if *ENABLE_NEW_RELIC {
            let (operation, table) = crate::sql_parser::parse_sql(&debug_query);
            execute_fn(&table, &operation, &debug_query, f)
        } else {
            f()
        }
    }

    fn query_by_name<T, U>(&self, source: &T) -> QueryResult<Vec<U>>
    where
        T: diesel::query_builder::QueryFragment<diesel::pg::Pg> + diesel::query_builder::QueryId,
        U: diesel::deserialize::QueryableByName<diesel::pg::Pg>,
    {
        let f = || self.conn.query_by_name(source);

        if *ENABLE_NEW_RELIC {
            let query = {
                let mut qb = diesel::pg::PgQueryBuilder::default();
                source.to_sql(&mut qb)?;
                qb.finish()
            };
            let (operation, table) = crate::sql_parser::parse_sql(&query);
            execute_fn(&table, &operation, &query, f)
        } else {
            f()
        }
    }

    fn execute_returning_count<T>(&self, source: &T) -> QueryResult<usize>
    where
        T: diesel::query_builder::QueryFragment<diesel::pg::Pg> + diesel::query_builder::QueryId,
    {
        let f = || self.conn.execute_returning_count(source);
        if *ENABLE_NEW_RELIC {
            let query = {
                let mut qb = diesel::pg::PgQueryBuilder::default();
                source.to_sql(&mut qb)?;
                qb.finish()
            };
            let (operation, table) = crate::sql_parser::parse_sql(&query);
            execute_fn(&table, &operation, &query, f)
        } else {
            f()
        }
    }

    fn transaction_manager(&self) -> &Self::TransactionManager {
        self.conn.transaction_manager()
    }
}
