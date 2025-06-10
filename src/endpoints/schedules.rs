use crate::client::JikanClient;
use crate::error::Result;
use crate::models::schedule::*;

impl JikanClient {
    pub async fn get_schedules(&self, filter: Option<&str>, sfw: Option<bool>, unapproved: Option<bool>, page: Option<u32>, limit: Option<u32>) -> Result<ScheduleResponse> {
        let mut params = Vec::new();
        
        if let Some(filter) = filter {
            params.push(format!("filter={}", filter));
        }
        if let Some(sfw) = sfw {
            params.push(format!("sfw={}", sfw));
        }
        if let Some(unapproved) = unapproved {
            params.push(format!("unapproved={}", unapproved));
        }
        if let Some(page) = page {
            params.push(format!("page={}", page));
        }
        if let Some(limit) = limit {
            params.push(format!("limit={}", limit));
        }
        
        let endpoint = if params.is_empty() {
            "schedules".to_string()
        } else {
            format!("schedules?{}", params.join("&"))
        };
        
        self.get(&endpoint).await
    }
} 