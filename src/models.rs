use super::schema::posts;
use super::schema::tags;
use super::schema::posts_tags;
use std::time::{SystemTime, UNIX_EPOCH};
use blog_grpc::blog;
use uuid::Uuid;

#[derive(Queryable, Associations)]
pub struct Post {
    pub id: Uuid,
    pub title: String,
    pub body: String,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
}

impl Post {
    pub fn into_response(&self) -> blog::Post {
        blog::Post {
            id: self.id.to_hyphenated().to_string(),
            title: self.title.to_owned(),
            body: self.body.to_owned(),
            tags: vec![],
            created_at: Some(blog::Timestamp {
                seconds: self.created_at.duration_since(UNIX_EPOCH).unwrap().as_secs(),
                nanos: self.created_at.duration_since(UNIX_EPOCH).unwrap().subsec_nanos(),
            }),
            updated_at: Some(blog::Timestamp {
                seconds: self.updated_at.duration_since(UNIX_EPOCH).unwrap().as_secs(),
                nanos: self.updated_at.duration_since(UNIX_EPOCH).unwrap().subsec_nanos(),
            }),
        }
    }
}

#[derive(Insertable)]
#[table_name="posts"]
pub struct NewPost {
    pub title: String,
    pub body: String,
}

pub struct UpdatePost {
    pub id: Uuid,
    pub title: String,
    pub body: String,
}

pub struct PaginatedPost {
    pub posts: Vec<Post>,
    pub next_page_token: Option<String>,
}
