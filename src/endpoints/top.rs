use crate::client::JikanClient;
use crate::error::Result;
use crate::models::top::*;

impl JikanClient {
    pub async fn get_top_anime(&self, page: Option<u32>, limit: Option<u32>, anime_type: Option<&str>, filter: Option<&str>) -> Result<TopAnimeResponse> {
        let mut params = Vec::new();
        
        if let Some(page) = page {
            params.push(format!("page={}", page));
        }
        if let Some(limit) = limit {
            params.push(format!("limit={}", limit));
        }
        if let Some(anime_type) = anime_type {
            params.push(format!("type={}", anime_type));
        }
        if let Some(filter) = filter {
            params.push(format!("filter={}", filter));
        }
        
        let endpoint = if params.is_empty() {
            "top/anime".to_string()
        } else {
            format!("top/anime?{}", params.join("&"))
        };
        
        self.get(&endpoint).await
    }

    pub async fn get_top_manga(&self, page: Option<u32>, limit: Option<u32>, manga_type: Option<&str>, filter: Option<&str>) -> Result<TopMangaResponse> {
        let mut params = Vec::new();
        
        if let Some(page) = page {
            params.push(format!("page={}", page));
        }
        if let Some(limit) = limit {
            params.push(format!("limit={}", limit));
        }
        if let Some(manga_type) = manga_type {
            params.push(format!("type={}", manga_type));
        }
        if let Some(filter) = filter {
            params.push(format!("filter={}", filter));
        }
        
        let endpoint = if params.is_empty() {
            "top/manga".to_string()
        } else {
            format!("top/manga?{}", params.join("&"))
        };
        
        self.get(&endpoint).await
    }

    pub async fn get_top_characters(&self, page: Option<u32>, limit: Option<u32>) -> Result<TopCharactersResponse> {
        let mut params = Vec::new();
        
        if let Some(page) = page {
            params.push(format!("page={}", page));
        }
        if let Some(limit) = limit {
            params.push(format!("limit={}", limit));
        }
        
        let endpoint = if params.is_empty() {
            "top/characters".to_string()
        } else {
            format!("top/characters?{}", params.join("&"))
        };
        
        self.get(&endpoint).await
    }

    pub async fn get_top_people(&self, page: Option<u32>, limit: Option<u32>) -> Result<TopPeopleResponse> {
        let mut params = Vec::new();
        
        if let Some(page) = page {
            params.push(format!("page={}", page));
        }
        if let Some(limit) = limit {
            params.push(format!("limit={}", limit));
        }
        
        let endpoint = if params.is_empty() {
            "top/people".to_string()
        } else {
            format!("top/people?{}", params.join("&"))
        };
        
        self.get(&endpoint).await
    }

    pub async fn get_top_reviews(&self, page: Option<u32>, review_type: Option<&str>) -> Result<TopReviewsResponse> {
        let mut params = Vec::new();
        
        if let Some(page) = page {
            params.push(format!("page={}", page));
        }
        if let Some(review_type) = review_type {
            params.push(format!("type={}", review_type));
        }
        
        let endpoint = if params.is_empty() {
            "top/reviews".to_string()
        } else {
            format!("top/reviews?{}", params.join("&"))
        };
        
        self.get(&endpoint).await
    }
} 