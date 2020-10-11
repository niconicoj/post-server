#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod schema;
pub mod models;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use uuid::Uuid;

// I don't like having this dependency here but It will work fine for the scope of the project.
use tonic::Status;

pub fn establish_connection() -> Result<PgConnection, Status> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL is not set");
    match PgConnection::establish(&database_url) {
        Ok(connection) => Ok(connection),
        Err(_) => Err(Status::unavailable("could not establish a connection to the database"))
    }
}

pub fn create_post(new_post: &models::NewPost, conn: &PgConnection) -> Result<models::Post, Status> {
    use schema::posts;
    match diesel::insert_into(posts::table)
        .values(new_post)
        .get_result(conn) {
            Ok(res) => Ok(res),
            Err(_) => Err(Status::internal("failed to save the post"))
        }
}

pub fn read_post(post_id: String, conn: &PgConnection) -> Result<models::Post, Status> {
    use schema::posts::dsl::*;

    let uuid = match Uuid::parse_str(post_id.as_str()) {
        Ok(uuid) => uuid,
        Err(_) => return Err(Status::invalid_argument("incorrectly formatted uuid.")),
    };
    
    match posts.find(uuid).first::<models::Post>(conn) {
        Ok(res) => Ok(res),
        Err(_) => Err(Status::not_found("requested post could not be found"))
    }
}
