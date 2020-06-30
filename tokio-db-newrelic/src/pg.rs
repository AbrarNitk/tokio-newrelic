use diesel::prelude::*;
use diesel;
use diesel::query_builder::QueryBuilder;

use crate::{TL_TRANSACTION, ENABLE_NEW_RELIC};

use newrelic::{DatastoreParamsBuilder, Datastore};

pub struct DebugConnection {
    pub conn: diesel::PgConnection,
}

pub type OConnection = DebugConnection;


lazy_static! {
    pub static ref PG_POOLS: antidote::RwLock<
        std::collections::HashMap<String, diesel::r2d2::Pool<r2d2_diesel::ConnectionManager<OConnection>>>,
    > = antidote::RwLock::new(std::collections::HashMap::new());
}


impl diesel::connection::SimpleConnection for OConnection {
    fn batch_execute(&self, query: &str) -> QueryResult<()> {
        self.conn.batch_execute(query)
    }
}

impl OConnection {
    fn new(url: &str) -> ConnectionResult<Self> {
        Ok(DebugConnection {
            conn: diesel::PgConnection::establish(url)?,
        })
    }
}


impl diesel::connection::Connection for OConnection {
    type Backend = diesel::pg::Pg;
    type TransactionManager = diesel::connection::AnsiTransactionManager;


    fn establish(url: &str) -> ConnectionResult<Self> {
        if *ENABLE_NEW_RELIC {
            let segment_params = DatastoreParamsBuilder::new(Datastore::Postgres)
                .collection("establish_connection")
                .build()
                .expect("Invalid data store parameters");
            TL_TRANSACTION.inner.with(|value| {
                value.borrow().as_ref().map_or_else(
                    || OConnection::new(url),
                    |trans| {
                        trans.as_ref().map_or_else(
                            || OConnection::new(url),
                            |t| t.datastore_segment(&segment_params, |_| OConnection::new(url)),
                        )
                    },
                )
            })
        } else {
            OConnection::new(url)
        }
    }

    fn execute(&self, query: &str) -> QueryResult<usize> {
        let (operation, table) = crate::sql_parser::parse_sql(&query);
//        crate::observe_fields::observe_string("query", &&query.replace("\"", ""));
//        crate::observe_span_id(&format!("db__{}__{}", operation, table.replace("\"", "")));
        self.conn.execute(query)
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
//        let (operation, table) = crate::sql_parse::parse_sql(&debug_query);
//        crate::observe_fields::observe_string("query", &debug_query.replace("\"", ""));
//        crate::observe_span_id(&format!("db__{}__{}", operation, table.replace("\"", "")));
        self.conn.query_by_index(query)
    }


    fn query_by_name<T, U>(&self, source: &T) -> QueryResult<Vec<U>>
        where
            T: diesel::query_builder::QueryFragment<diesel::pg::Pg> + diesel::query_builder::QueryId,
            U: diesel::deserialize::QueryableByName<diesel::pg::Pg>,
    {
        let query = {
            let mut qb = diesel::pg::PgQueryBuilder::default();
            source.to_sql(&mut qb)?;
            qb.finish()
        };
//        let (operation, table) = crate::sql_parse::parse_sql(&query);
//        crate::observe_fields::observe_string("query", &query.replace("\"", ""));
//        crate::observe_span_id(&format!("db__{}__{}", operation, table.replace("\"", "")));
        self.conn.query_by_name(source)
    }


    fn execute_returning_count<T>(&self, source: &T) -> QueryResult<usize>
        where
            T: diesel::query_builder::QueryFragment<diesel::pg::Pg> + diesel::query_builder::QueryId,
    {
        let query = {
            let mut qb = diesel::pg::PgQueryBuilder::default();
            source.to_sql(&mut qb)?;
            qb.finish()
        };
//        let (operation, table) = crate::sql_parse::parse_sql(&query);
//        crate::observe_fields::observe_string("query", &query.replace("\"", ""));
//        crate::observe_span_id(&format!("db__{}__{}", operation, table.replace("\"", "")));
        self.conn.execute_returning_count(source)
    }

    fn transaction_manager(&self) -> &Self::TransactionManager {
        self.conn.transaction_manager()
    }
}
