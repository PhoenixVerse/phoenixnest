use std::collections::BTreeMap;

use candid::Principal;

use super::{
    domain::{
        PostProfile, PostId, PostCreateCommand, Timestamp, PostStatus, PostEditCommand, PostPage, PostPageQuery, PostChangeStatusCommand,
    }, 
    error::PostError
};


#[derive(Debug, Default)]
pub struct PostService {
    pub posts: BTreeMap<PostId, PostProfile>,
}

impl PostService {
    pub fn create_post(&mut self, cmd: PostCreateCommand, id: u64, caller: Principal, now: Timestamp) -> Option<u64> {
        match self.posts.get(&id) {
            Some(_) => None,
            None => {
                self.posts.insert(
                    id.into(),
                    cmd.build_profile(
                        id,
                        caller,
                        PostStatus::Enable,
                        now
                    )
                );                
                Some(id)
            }
        }   
    }  

    pub fn edit_post(&mut self, cmd: PostEditCommand) -> Option<bool> {
        self.posts.get_mut(&cmd.id).map(|profile| {
            cmd.merge_profile(profile);
        }).map(|_| true)
    }

    pub fn change_post_status(&mut self, cmd: PostChangeStatusCommand, caller: Principal, now: Timestamp) -> Result<bool, PostError> {
        if let Some(profile) = self.posts.get_mut(&cmd.id) {
            if !profile.is_active() {
                return Err(PostError::PostAlreadyCompleted);
            }

            let new_status = cmd.status.parse::<PostStatus>().unwrap();
            
            
            profile.status = new_status;

            Ok(true)
        } else {
            Err(PostError::PostNotFound)
        }
    }

    pub fn delete_post(&mut self, id: PostId) -> Option<bool> {
        self.posts.remove(&id).map(|_| true)
    }
    
    pub fn get_post(&self, id: PostId) -> Option<PostProfile> {
        self.posts.get(&id).cloned()
    }

    pub fn page_posts(&self, query_args: &PostPageQuery) -> PostPage {
        let data: Vec<PostProfile> = self.posts
            .iter()
            .filter(|(_, q)| query_args.querystring.is_empty() || (q.title.contains(&query_args.querystring) || q.content.content.contains(&query_args.querystring)))
            .skip(query_args.page_num * query_args.page_size)
            .take(query_args.page_size)
            .map(|(_, q)| q.clone())
            .collect();

        let total_count = self.posts.len();
        
        PostPage {
            data,
            page_size: query_args.page_size,
            page_num: query_args.page_num,
            total_count,
        }
    }

}

#[cfg(test)]
mod tests {

    use crate::post::domain::RichText;

    use super::*;

    #[test] 
    fn add_comment_comment_should_work() {
        let mut svc = PostService::default();
        let id = 10001u64;
        let caller = Principal::anonymous();
        let now = 20220516u64;
        let create_cmd = PostCreateCommand { 
            title: "james title".to_string(),
            content: RichText {
                content: "james content".to_string(),
                format: "md".to_string(),
            },
            category: "Tech".to_string(),
            photos: vec![30, 20],
            
        };

        let res1 = svc.create_post(create_cmd, id, caller, now);

        assert_eq!(res1.unwrap(), 10001u64);

        let res3 = svc.get_post(id).unwrap();
        assert_eq!(res3.title, "james title".to_string());

    }
}