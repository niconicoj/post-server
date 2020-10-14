use tonic::{Request, Response, Status};

use blog_grpc::blog::{
    CreatePostRequest,
    DeletePostRequest,
    ReadPostRequest,
    UpdatePostRequest,
    ListPostRequest,
    Post,
    ListPostResponse,
    Empty,
    Timestamp,
    post_service_server::PostService,
};

use super::post_server::{
    models, 
    create_post, 
    read_post, 
    update_post, 
    delete_post,
    list_post,
    establish_connection,
};

use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Default)]
pub struct MyPostService {}

#[tonic::async_trait]
impl PostService for MyPostService {
    async fn create_post(
        &self, 
        request: Request<CreatePostRequest>
    ) -> Result<Response<Post>, Status> {
        println!("create request : {:?}", request);

        let reply = match request.into_inner().post {
            Some(post) => {

                let new_post = models::NewPost {
                    title: post.title,
                    body: post.body,
                };
                let conn = establish_connection()?;
                let saved_post = create_post(&new_post, &conn)?;
                let created_at = match (saved_post.created_at as SystemTime).duration_since(UNIX_EPOCH) {
                    Ok(ts) => ts,
                    Err(_) => return Err(Status::internal("internal server error")),
                };
                let updated_at = match (saved_post.updated_at as SystemTime).duration_since(UNIX_EPOCH) {
                    Ok(ts) => ts,
                    Err(_) => return Err(Status::internal("internal server error")),
                };

                Ok(Response::new(Post {
                    id: saved_post.id.to_hyphenated().to_string(),
                    title: saved_post.title,
                    body: saved_post.body,
                    tags: post.tags,
                    created_at: Some(Timestamp {
                        seconds: created_at.as_secs(),
                        nanos: created_at.subsec_nanos(),
                    }),
                    updated_at: Some(Timestamp {
                        seconds: updated_at.as_secs(),
                        nanos: updated_at.subsec_nanos(),
                    }),
                }))
            },
            None => Err(Status::invalid_argument("something is wrong with the request.")),
        };
        reply
    }
    async fn read_post(&self, request: Request<ReadPostRequest>) -> Result<Response<Post>, Status> {
        println!("read request : {:?}",request);
        let conn = establish_connection()?;

        let post = read_post(request.into_inner().id, &conn)?;
        let created_at = match (post.created_at as SystemTime).duration_since(UNIX_EPOCH) {
            Ok(ts) => ts,
            Err(_) => return Err(Status::internal("internal server error")),
        };
        let updated_at = match (post.updated_at as SystemTime).duration_since(UNIX_EPOCH) {
            Ok(ts) => ts,
            Err(_) => return Err(Status::internal("internal server error")),
        };

        Ok(Response::new(Post {
            id: post.id.to_hyphenated().to_string(),
            title: post.title,
            body: post.body,
            tags: vec![],
            created_at: Some(Timestamp {
                seconds: created_at.as_secs(),
                nanos: created_at.subsec_nanos(),
            }),
            updated_at: Some(Timestamp {
                seconds: updated_at.as_secs(),
                nanos: updated_at.subsec_nanos(),
            }),
        }))
    }

    async fn update_post(&self, request: Request<UpdatePostRequest>) -> Result<Response<Post>, Status> {
        println!("update request : {:?}",request);

        let reply = match request.into_inner().post {
            Some(post) => {

                let id = match uuid::Uuid::parse_str(post.id.as_str()) {
                    Ok(id) => id,
                    Err(_) => return Err(Status::invalid_argument("incorrect uuid format")),
                };

                let updated_post = models::UpdatePost {
                    id,
                    title: post.title,
                    body: post.body,
                };
                let conn = establish_connection()?;
                let saved_post = update_post(&updated_post, &conn)?;
                let created_at = match (saved_post.created_at as SystemTime).duration_since(UNIX_EPOCH) {
                    Ok(ts) => ts,
                    Err(_) => return Err(Status::internal("internal server error")),
                };
                let updated_at = match (saved_post.updated_at as SystemTime).duration_since(UNIX_EPOCH) {
                    Ok(ts) => ts,
                    Err(_) => return Err(Status::internal("internal server error")),
                };

                Ok(Response::new(Post {
                    id: saved_post.id.to_hyphenated().to_string(),
                    title: saved_post.title,
                    body: saved_post.body,
                    tags: post.tags,
                    created_at: Some(Timestamp {
                        seconds: created_at.as_secs(),
                        nanos: created_at.subsec_nanos(),
                    }),
                    updated_at: Some(Timestamp {
                        seconds: updated_at.as_secs(),
                        nanos: updated_at.subsec_nanos(),
                    }),
                }))
            },
            None => Err(Status::invalid_argument("something is wrong with the request.")),
        };
        reply
    }

    async fn delete_post(&self, request: Request<DeletePostRequest>) -> Result<Response<Empty>, Status> {
        println!("delete request : {:?}",request);
        let conn = establish_connection()?;

        match delete_post(request.into_inner().id, &conn) {
            Ok(_) => Ok(Response::new(Empty {})),
            Err(err) => Err(err),
        }
    }

    async fn list_post(&self, request: Request<ListPostRequest>) -> Result<Response<ListPostResponse>, Status> {
        println!("list request : {:?}",request);
        let conn = establish_connection()?;
        let req = request.into_inner();
        match list_post(req.page_token, req.page_size, &conn) {
            Ok(res) => {
                Ok(Response::new(
                    ListPostResponse {
                        posts: res.posts.iter()
                            .map(|p| p.into_response()).collect(),
                            next_page_token: match res.next_page_token {
                                Some(token) => token,
                                None => "".to_string()
                            }
                    }
                ))
            },
            Err(_) => Err(Status::invalid_argument("something is wrong with the request.")),
        }
    }


}
