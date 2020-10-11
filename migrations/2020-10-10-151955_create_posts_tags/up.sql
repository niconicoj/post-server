CREATE TABLE posts_tags (
    id UUID PRIMARY KEY,
    post_id UUID references posts(id),
    tag_id UUID references tags(id)
)
