use std::collections::HashMap;
use tokio::sync::Mutex;
use crate::model::{CreatePostRequest, Post, UpdatePostRequest};

pub struct InMemoryPostStore {
    pub counter: i64,
    pub items: HashMap<i64, Post>,
}

pub struct InMemoryPostService {
    data: Mutex<InMemoryPostStore>,
}

impl Default for InMemoryPostService {
    fn default() -> Self {
        Self {
            data: Mutex::new(InMemoryPostStore {
                counter: 0,
                items: Default::default(),
            }),
        }
    }
}

impl PostService for InMemoryPostService {
    async fn get_all_posts(&self) -> anyhow::Result<Vec<Post>> {
        let data = self.data.lock().await;
        Ok(data.items.values().map(|post| (*post).clone()).collect())
    }
    async fn get_post_by_id(&self, id: i64) -> anyhow::Result<Post> {
        let data = self.data.lock().await;

        match data.items.get(&id) {
            Some(post) => Ok((*post).clone()),
            None => anyhow::bail!("Post not found: {}", id),
        }
    }
    async fn get_post_by_slug(&self, slug: &str) -> anyhow::Result<Post> {
        let data = self.data.lock().await;
        for (_id, post) in data.items.iter() {
            if post.slug == slug {
                return Ok(post.clone());
            }
        }

        anyhow::bail!("Post not found: {}", slug)
    }
    async fn create_post(&self, req: CreatePostRequest) -> anyhow::Result<Post> {
        let mut data = self.data.lock().await;
        data.counter += 1;
        let ts = chrono::offset::Utc::now();
        let post = Post {
            id: data.counter,
            author_id: req.author_id,
            slug: req.slug,
            title: req.title,
            content: req.content,
            status: req.status,
            created: ts,
            updated: ts,
        };

        data.items.insert(post.id, post);

        match data.items.get(&data.counter) {
            None => {
                anyhow::bail!("Post not found: {}", data.counter)
            }
            Some(post) => Ok(post.clone()),
        }
    }

    async fn update_post(&self, req: UpdatePostRequest) -> anyhow::Result<Post> {
        let mut data = self.data.lock().await;
        let post = data
            .items
            .get_mut(&req.id)
            .ok_or(anyhow::anyhow!("Post not found: {}", req.id))?;

        post.slug = req.slug;
        post.title = req.title;
        post.content = req.content;
        post.status = req.status;

        match data.items.get(&data.counter) {
            None => {
                anyhow::bail!("Post not found: {}", req.id)
            }
            Some(post) => Ok(post.clone()),
        }
    }

    async fn delete_post(&self, id: i64) -> anyhow::Result<()> {
        let mut data = self.data.lock().await;
        match data.items.remove(&id) {
            None => {
                anyhow::bail!("Post not found: {}", id)
            }
            Some(_) => Ok(()),
        }
    }
}
#[allow(async_fn_in_trait)]
pub trait PostService {
    async fn get_all_posts(&self) -> anyhow::Result<Vec<Post>>;
    async fn get_post_by_id(&self, id: i64) -> anyhow::Result<Post>;
    async fn get_post_by_slug(&self, name: &str) -> anyhow::Result<Post>;
    async fn create_post(&self, req: CreatePostRequest) -> anyhow::Result<Post>;
    async fn update_post(&self, req: UpdatePostRequest) -> anyhow::Result<Post>;
    async fn delete_post(&self, id: i64) -> anyhow::Result<()>;
}