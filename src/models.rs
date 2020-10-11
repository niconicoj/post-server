use super::schema::posts;
use super::schema::tags;
use super::schema::posts_tags;
use std::time::SystemTime;

use uuid::Uuid;

#[derive(Queryable, Associations)]
pub struct Post {
    pub id: Uuid,
    pub title: String,
    pub body: String,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
}

#[derive(Insertable)]
#[table_name="posts"]
pub struct NewPost {
    pub title: String,
    pub body: String,
}
