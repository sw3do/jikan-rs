use crate::client::JikanClient;
use crate::error::Result;
use crate::models::watch::*;

impl JikanClient {
    pub async fn get_watch_recent_episodes(&self, page: Option<u32>) -> Result<WatchEpisodesResponse> {
        let mut endpoint = "watch/episodes".to_string();
        if let Some(page) = page {
            endpoint.push_str(&format!("?page={}", page));
        }
        self.get(&endpoint).await
    }

    pub async fn get_watch_popular_episodes(&self, page: Option<u32>) -> Result<WatchEpisodesResponse> {
        let mut endpoint = "watch/episodes/popular".to_string();
        if let Some(page) = page {
            endpoint.push_str(&format!("?page={}", page));
        }
        self.get(&endpoint).await
    }

    pub async fn get_watch_recent_promos(&self, page: Option<u32>) -> Result<WatchPromosResponse> {
        let mut endpoint = "watch/promos".to_string();
        if let Some(page) = page {
            endpoint.push_str(&format!("?page={}", page));
        }
        self.get(&endpoint).await
    }

    pub async fn get_watch_popular_promos(&self, page: Option<u32>) -> Result<WatchPromosResponse> {
        let mut endpoint = "watch/promos/popular".to_string();
        if let Some(page) = page {
            endpoint.push_str(&format!("?page={}", page));
        }
        self.get(&endpoint).await
    }
} 