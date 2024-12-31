use uuid::Uuid;
use std::fmt;

// Represent a blog post
#[derive(Clone, Debug)]
pub struct Post {
    uuid: Uuid,
    title: String,
    content: String,
    published: bool,
}

impl Post {
    pub fn new(title: String, content: String) -> Post {
        Post {
            uuid: Uuid::new_v4(),  // Generate random UUID
            title,
            content,
            published: false,
        }
    }

    pub fn uuid(&self) -> Uuid {
        self.uuid
    }

    pub fn publish(&mut self) {
        self.published = true;
    }
}

// Display trait for pretty printing
impl fmt::Display for Post {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Post(uuid={}, title={}, published={})",
               self.uuid, self.title, self.published)
    }
}

#[derive(Clone, Debug)]
pub struct Database {
    posts: Vec<Post>,
}

impl Database {
    // Create new empty database
    pub fn new() -> Database {
        Database {
            posts: vec![]
        }
    }

    // Add post to database
    pub fn add_post(&mut self, post: Post) {
        self.posts.push(post);
    }

    // List all posts with index
    pub fn list_posts(&self) -> () {
        for (index, post) in self.posts.iter().enumerate() {
            println!("Post {}: {}", index + 1, post);
        }
    }

    // Find post by UUID, returns Option
    pub fn find_post_by_uuid(&self, uuid: Uuid) -> Option<&Post> {
        self.posts.iter().find(|post| post.uuid() == uuid)
    }

    // Delete post by UUID, returns success
    pub fn delete_post(&mut self, uuid: Uuid) -> bool {
        if let Some(index) = self.posts.iter().position(|post| post.uuid() == uuid) {
            self.posts.remove(index);
            true
        } else {
            false
        }
    }

    // Get count of posts
    pub fn post_count(&self) -> usize {
        self.posts.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_database_operations() {
        let mut db = Database::new();
        assert_eq!(db.post_count(), 0);

        let post = Post::new("Test".to_string(), "Content".to_string());
        let uuid = post.uuid();
        db.add_post(post);

        assert_eq!(db.post_count(), 1);
        assert!(db.find_post_by_uuid(uuid).is_some());

        assert!(db.delete_post(uuid));
        assert_eq!(db.post_count(), 0);
    }
}