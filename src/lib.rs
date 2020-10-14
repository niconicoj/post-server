#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod schema;
pub mod models;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::{
    env,
    time::SystemTime,
};
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

pub fn update_post(updated_post: &models::UpdatePost, conn: &PgConnection) -> Result<models::Post, Status> {
    use schema::posts::dsl::*;

    match diesel::update(posts.find(updated_post.id))
        .set((
                title.eq(&updated_post.title),
                body.eq(&updated_post.body),
        ))
        .get_result(conn) {
            Ok(res) => Ok(res),
            Err(_) => Err(Status::invalid_argument("could not perform update")),
        }
}

pub fn delete_post(post_id: String, conn: &PgConnection) -> Result<(), Status> {
    use schema::posts::dsl::*;

    let uuid = match Uuid::parse_str(post_id.as_str()) {
        Ok(uuid) => uuid,
        Err(_) => return Err(Status::invalid_argument("incorrectly formatted uuid.")),
    };

    match diesel::delete(posts.find(uuid)).execute(conn) {
        Ok(_) => Ok(()),
        Err(_) => Err(Status::invalid_argument("could not perform deletion.")),
    }
}

pub fn list_post(page_token: String, page_size: u32, conn: &PgConnection) -> Result<models::PaginatedPost, Status> {
    use schema::posts::dsl::*;
    let result = match page_token.is_empty() {
        true => posts
            .order(created_at.desc())
            .limit(page_size as i64)
            .load::<models::Post>(conn),
        false => {
            let uuid = match Uuid::parse_str(page_token.as_str()) {
                Ok(uuid) => uuid,
                Err(_) => return Err(Status::invalid_argument("incorrectly formatted uuid.")),
            };
            let first_post = match posts.select(created_at).find(uuid).first::<SystemTime>(conn) {
                Ok(sys_time) => sys_time,
                Err(_) => return Err(Status::not_found("could not find a post to paginate from.")),
            };
            posts
                .order(created_at.desc())
                .limit(page_size as i64)
                .filter(created_at.lt(first_post))
                .load::<models::Post>(conn)
        }
    };

    match result {
        Ok(res) => {
            match res.len() == page_size as usize {
                true => {
                    let next_page_token = res.last().unwrap().id.to_string();

                    Ok( models::PaginatedPost {
                        posts: res,
                        next_page_token: Some(next_page_token),
                    } )
                },
                false => {
                    Ok( models::PaginatedPost {
                        posts: res,
                        next_page_token: None,
                    } )
                },
            }
        },
        Err(_) => Err(Status::invalid_argument("could not perform list.")),
    }
}
