use diesel::pg::PgConnection;
use dotenv::dotenv;
use r2d2_diesel::ConnectionManager;
use rocket_contrib::Json;
use std::env;
use views::EncodablePackage;
use r2d2;

use self::db_guard::DbConn;

mod db_guard;

// An alias to the type for a pool of Diesel postgres connections.
pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn init_pool() -> Pool {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder().build(manager).expect("Failed to create database pool")
}

#[get("/")]
pub fn index() -> &'static str {
    "Hello, world handler!"
}

#[get("/list")]
pub fn list(conn: DbConn) -> Option<Json<EncodablePackage>> {
    let package = super::search_packages(&*conn);

    Some(Json(package?.minimal_encodable()))
}
