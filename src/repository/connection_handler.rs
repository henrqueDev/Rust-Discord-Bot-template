use std::env;
use crate::dotenv;
use diesel::{pg::PgConnection, Connection};

pub fn get_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let connection = PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));
    
    return connection;
}