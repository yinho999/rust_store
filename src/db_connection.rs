use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PoolError, PooledConnection};

use dotenv::dotenv;
use std::env;

// Adding connection pool Type
pub type PgPool = Pool<ConnectionManager<PgConnection>>;
pub type PgPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

// Init connection pool
pub fn init_pool(database_url: &str) -> Result<PgPool, PoolError> {
    let manager = ConnectionManager::<PgConnection>::new(database_url.to_string());
    Pool::builder().build(manager)
}

// Establish a connection to the database.
pub fn establish_connection() -> PgPool {
    // Load the environment variables from the .env file.
    dotenv().ok();

    // Get the database URL from the environment variables.
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Create a connection to the database.
    init_pool(&database_url).expect("Error connecting to database")
}
