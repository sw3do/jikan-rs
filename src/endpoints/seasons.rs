use crate::client::JikanClient;
use crate::error::Result;
use crate::models::seasonal::*;

impl JikanClient {
    pub async fn get_season_now(&self, page: Option<u32>, limit: Option<u32>, filter: Option<&str>, sfw: Option<bool>, unapproved: Option<bool>, continuing: Option<bool>) -> Result<SeasonNowResponse> {
        let mut params = Vec::new();
        
        if let Some(page) = page {
            params.push(format!("page={}", page));
        }
        if let Some(limit) = limit {
            params.push(format!("limit={}", limit));
        }
        if let Some(filter) = filter {
            params.push(format!("filter={}", filter));
        }
        if let Some(sfw) = sfw {
            params.push(format!("sfw={}", sfw));
        }
        if let Some(unapproved) = unapproved {
            params.push(format!("unapproved={}", unapproved));
        }
        if let Some(continuing) = continuing {
            params.push(format!("continuing={}", continuing));
        }
        
        let endpoint = if params.is_empty() {
            "seasons/now".to_string()
        } else {
            format!("seasons/now?{}", params.join("&"))
        };
        
        self.get(&endpoint).await
    }

    pub async fn get_season(&self, year: u16, season: &str, page: Option<u32>, limit: Option<u32>, filter: Option<&str>, sfw: Option<bool>, unapproved: Option<bool>, continuing: Option<bool>) -> Result<SeasonalAnimeResponse> {
        let mut params = Vec::new();
        
        if let Some(page) = page {
            params.push(format!("page={}", page));
        }
        if let Some(limit) = limit {
            params.push(format!("limit={}", limit));
        }
        if let Some(filter) = filter {
            params.push(format!("filter={}", filter));
        }
        if let Some(sfw) = sfw {
            params.push(format!("sfw={}", sfw));
        }
        if let Some(unapproved) = unapproved {
            params.push(format!("unapproved={}", unapproved));
        }
        if let Some(continuing) = continuing {
            params.push(format!("continuing={}", continuing));
        }
        
        let endpoint = if params.is_empty() {
            format!("seasons/{}/{}", year, season)
        } else {
            format!("seasons/{}/{}?{}", year, season, params.join("&"))
        };
        
        self.get(&endpoint).await
    }

    pub async fn get_season_upcoming(&self, page: Option<u32>, limit: Option<u32>, filter: Option<&str>, sfw: Option<bool>, unapproved: Option<bool>) -> Result<SeasonUpcomingResponse> {
        let mut params = Vec::new();
        
        if let Some(page) = page {
            params.push(format!("page={}", page));
        }
        if let Some(limit) = limit {
            params.push(format!("limit={}", limit));
        }
        if let Some(filter) = filter {
            params.push(format!("filter={}", filter));
        }
        if let Some(sfw) = sfw {
            params.push(format!("sfw={}", sfw));
        }
        if let Some(unapproved) = unapproved {
            params.push(format!("unapproved={}", unapproved));
        }
        
        let endpoint = if params.is_empty() {
            "seasons/upcoming".to_string()
        } else {
            format!("seasons/upcoming?{}", params.join("&"))
        };
        
        self.get(&endpoint).await
    }

    pub async fn get_seasons_archive(&self) -> Result<SeasonArchiveResponse> {
        self.get("seasons").await
    }
} 