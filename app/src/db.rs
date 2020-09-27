use std::ops::Deref;
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Request, State, Outcome};
use diesel::mysql::MysqlConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
extern crate dotenv;
// An alias to the type for a pool of Sqlite Connection
pub type MySqlPool = Pool<ConnectionManager<MysqlConnection>>;
use std::env;


// The URL to the database, set via the `DATABASE_URL` environment variable.
static DATABASE_URL: &str = env!("DATABASE_URL");
/// Initialize the database pool.
pub fn connect() -> MySqlPool {
    let manager = ConnectionManager::<MysqlConnection>::new(DATABASE_URL);
    Pool::new(manager).expect("Failed to create pool")
}

// Connection request guard type: a wrapper around an r2d2 pooled connection.
pub struct Connection(pub PooledConnection<ConnectionManager<MysqlConnection>>);

/// Attempts to retrieve a single connection from the managed database pool. If
/// no pool is currently managed, fails with an `InternalServerError` status. If
/// no connections are available, fails with a `ServiceUnavailable` status.
impl<'a, 'r> FromRequest<'a, 'r> for Connection {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let pool = request.guard::<State<MySqlPool>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(Connection(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ()))
        }
    }
}

// For the convenience of using an &Connection as an &Sqlite
impl Deref for Connection {
    type Target = MysqlConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}