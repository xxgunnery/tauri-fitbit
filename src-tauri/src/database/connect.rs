use core::time::Duration;
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use std::env;

pub type PostgresPool = Pool<ConnectionManager<PgConnection>>;

pub fn establish_connection() -> PostgresPool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("{:?}", database_url);
    let manager = ConnectionManager::<PgConnection>::new(&database_url);

    Pool::builder()
        .connection_timeout(Duration::new(5, 0))
        .build(manager)
        .expect("Failed to create pool.")
}
