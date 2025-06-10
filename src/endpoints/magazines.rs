use crate::client::JikanClient;
use crate::error::Result;
use crate::models::magazine::*;

impl JikanClient {
    pub async fn get_magazines(&self, page: Option<u32>, limit: Option<u32>, query: Option<&str>, order_by: Option<&str>, sort: Option<&str>, letter: Option<&str>) -> Result<MagazinesResponse> {
        let mut params = Vec::new();
        
        if let Some(page) = page {
            params.push(format!("page={}", page));
        }
        if let Some(limit) = limit {
            params.push(format!("limit={}", limit));
        }
        if let Some(query) = query {
            params.push(format!("q={}", urlencoding::encode(query)));
        }
        if let Some(order_by) = order_by {
            params.push(format!("order_by={}", order_by));
        }
        if let Some(sort) = sort {
            params.push(format!("sort={}", sort));
        }
        if let Some(letter) = letter {
            params.push(format!("letter={}", letter));
        }
        
        let endpoint = if params.is_empty() {
            "magazines".to_string()
        } else {
            format!("magazines?{}", params.join("&"))
        };
        
        self.get(&endpoint).await
    }
} 