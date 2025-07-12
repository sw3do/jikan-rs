use crate::client::JikanClient;
use crate::error::Result;
use crate::models::top::TopReviewsResponse;

impl JikanClient {
    pub async fn get_recent_anime_reviews(&self, page: Option<u32>) -> Result<TopReviewsResponse> {
        let endpoint = if let Some(page) = page {
            format!("reviews/anime?page={page}")
        } else {
            "reviews/anime".to_string()
        };

        self.get(&endpoint).await
    }

    pub async fn get_recent_manga_reviews(&self, page: Option<u32>) -> Result<TopReviewsResponse> {
        let endpoint = if let Some(page) = page {
            format!("reviews/manga?page={page}")
        } else {
            "reviews/manga".to_string()
        };

        self.get(&endpoint).await
    }
}
