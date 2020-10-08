use tonic::{transport::Server, Request, Response, Status};

use blog_grpc::blog::{
    Post,
    CreatePostRequest,
    Timestamp,
    post_service_server::{PostService, PostServiceServer},
};

use std::time::SystemTime;

#[derive(Debug, Default)]
pub struct MyPostService {}

#[tonic::async_trait]
impl PostService for MyPostService {
    async fn create_post(
        &self, 
        request: Request<CreatePostRequest>
    ) -> Result<Response<Post>, Status> {
        println!("Got a request: {:?}", request);

        let timestamp = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).expect("failed to get a timestamp !");

        let reply = match request.into_inner().post {
            Some(post) => {
                Ok(Response::new(Post {
                    title: post.title,
                    body: post.body,
                    tags: post.tags,
                    created_at: Some(Timestamp {
                        seconds: timestamp.as_secs(),
                        nanos: timestamp.subsec_nanos()
                    }),
                    updated_at: Some(Timestamp {
                        seconds: timestamp.as_secs(),
                        nanos: timestamp.subsec_nanos()
                    }),
                }))
            },
            None => Err(Status::invalid_argument("something is wrong with the request.")),
        };
        reply
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let post_service = MyPostService::default();

    Server::builder()
        .add_service(PostServiceServer::new(post_service))
        .serve(addr)
        .await?;

    Ok(())
}

