use std::{
    collections::VecDeque, 
    string::ParseError, 
    str::FromStr
};

use candid::{CandidType, Deserialize, Principal};

pub type PostId = u64;
pub type Timestamp = u64;

#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct PostProfile {
    pub id: PostId,
    pub author: Principal,
    pub title: String,
    pub content: RichText,
    pub category: Category,
    pub photos: Vec<u64>,
    pub likes_count: u64,
    pub status: PostStatus,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,

}

impl PostProfile {
    pub fn new(id: u64, author: Principal, title: String, content: RichText, category: Category, photos: Vec<u64>, status: PostStatus, created_at: Timestamp) -> Self {
        Self {
            id,
            author,
            title,
            content,
            category,
            photos,
            likes_count: 0,
            status,
            created_at,
            updated_at: created_at,
        }
    }

    pub fn is_active(&self) -> bool {
        self.status == PostStatus::Enable
    }
}

#[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
pub struct RichText {
    pub content: String,
    pub format: String,
}

#[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
pub enum  Category {
    Tech,
    Law,
    Other,
}   

impl FromStr for Category {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "law" => Ok(Category::Law),
            "tech" => Ok(Category::Tech),
            _ => Ok(Category::Other)
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
pub enum PostStatus {
    Enable,
    Completed,
    Closed,
}

impl FromStr for PostStatus {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "completed" => Ok(PostStatus::Completed),
            "closed" => Ok(PostStatus::Closed),
            _ => Ok(PostStatus::Enable),
        }
    }
}

#[derive(Debug, Clone, CandidType, Deserialize)]
pub enum EventStatus {
    Enable,
    Disable,
}

#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct PostCreateCommand {
    pub title: String,
    pub content: RichText,
    pub category: String,
    pub photos: Vec<u64>,
}

impl PostCreateCommand {
    pub fn build_profile(self, id: u64, owner: Principal, status: PostStatus, now: Timestamp) -> PostProfile {
        PostProfile::new(id, owner, self.title, self.content, self.category.parse::<Category>().unwrap(), self.photos, status, now)
    }

}

#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct PostEditCommand {
    pub id: u64,
    title: String,
    content: RichText,
    category: String,
    photos: Vec<u64>,
    status: PostStatus,
}

impl PostEditCommand {
    pub fn merge_profile(self, profile: &mut PostProfile) {
        assert!(self.id == profile.id);

        profile.title = self.title;
        profile.content = self.content;
        profile.category = self.category.parse::<Category>().unwrap();
        profile.photos = self.photos;
        profile.status = self.status;
    }
}

#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct PostIdCommand {
    pub id: u64,
}

#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct PostChangeStatusCommand {
    pub id: u64,
    pub status: String,
    pub description: String,
}

#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct PostPage {
    pub data: Vec<PostProfile>,
    pub page_size: usize,
    pub page_num: usize,
    pub total_count: usize,
}

#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct PostPageQuery {
    pub page_size: usize,
    pub page_num: usize,
    pub querystring: String,
}
