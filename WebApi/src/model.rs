use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Clone, Debug, RustcEncodeable, RustcDecodable)]
pub struct Post {
    title: String,
    body: String,
    author: String,
    datetime: DateTime<Utc>,
    uuid: Uuid,
}

impl Post {
    /// Creates a new Post instance
    pub fn new(title: &str, body: &str, author: &str, datetime: DateTime<Utc>, uuid: Uuid) -> Post {
        Post {
            title: title.to_string(),
            body: body.to_string(),
            author: author.to_string(),
            datetime,
            uuid,
        }
    }

    /// Retrieves the UUID of the Post
    pub fn uuid(&self) -> Uuid {
        self.uuid
    }
}
