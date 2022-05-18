
use candid::{CandidType, Deserialize, Principal};
use std::collections::BTreeMap;
use std::iter::FromIterator;

use crate::env::{Environment, CanisterEnvironment, EmptyEnvironment};

use crate::post::domain::PostId;


use crate::post::{
    PostService,
    domain::PostProfile,
};

#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct DaoDataStorage {
    pub id: u64,    
    pub posts: Vec<PostProfile>,
}

impl From<DaoContext> for DaoDataStorage {
    fn from(state: DaoContext) -> Self {
        let id = state.id;
        let posts = Vec::from_iter(state.post_service.posts
            .iter()
            .map(|(_k, v)| (v.clone())));   

        Self {
            id,
            
            posts,
        }
    }
}

pub struct DaoContext {
    pub env: Box<dyn Environment>,
    pub id: u64,
    pub post_service: PostService,
}

impl Default for DaoContext {
    fn default() -> Self {
        Self {
            env: Box::new(EmptyEnvironment {}),
            id: 10001,
            post_service: PostService::default(),
        }
    }
}

impl From<DaoDataStorage> for DaoContext {
    fn from(payload: DaoDataStorage) -> Self {
        
        let posts: BTreeMap<PostId, PostProfile> = payload
            .posts
            .into_iter()
            .map(|p| (p.id, p))
            .collect();

        Self {
            env: Box::new(CanisterEnvironment {}),
            id: payload.id,
            
            post_service: PostService { posts },
        }
    }
}

#[cfg(test)]
mod tests {
    
}
