lazy_static! {
    pub static ref PG_POOLS: antidote::RwLock<
        std::collections::HashMap<
            String,
            diesel::r2d2::Pool<r2d2_diesel::ConnectionManager<crate::pg::NConnection>>,
        >,
    > = antidote::RwLock::new(std::collections::HashMap::new());
}

fn _connection_pool<T: Into<String>>(
    url: T,
) -> r2d2::Pool<r2d2_diesel::ConnectionManager<crate::pg::NConnection>> {
    let manager = r2d2_diesel::ConnectionManager::<crate::pg::NConnection>::new(url);
    r2d2::Pool::builder()
        .max_size(15)
        .build(manager)
        .expect("Fail to create Diesel Connection Pool")
}

pub fn connection_with_url(
    db_url: &str,
) -> r2d2::PooledConnection<r2d2_diesel::ConnectionManager<crate::pg::NConnection>> {
    {
        if let Some(pool) = PG_POOLS.read().get(&db_url.to_string()) {
            return pool.get().unwrap();
        }
    }
    match PG_POOLS.write().entry(db_url.to_string()) {
        std::collections::hash_map::Entry::Vacant(e) => {
            let conn_pool = _connection_pool(db_url);
            let conn = conn_pool.get().unwrap();
            e.insert(conn_pool);
            conn
        }
        std::collections::hash_map::Entry::Occupied(e) => e.get().get().unwrap(),
    }
}

pub fn connection() -> r2d2::PooledConnection<r2d2_diesel::ConnectionManager<crate::pg::NConnection>>
{
    connection_with_url(&std::env::var("PG_DATABASE_URL").expect("DATABASE_URL not set"))
}
