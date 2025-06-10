use crate::client::JikanClient;
use crate::error::Result;
use crate::models::search::*;

impl JikanClient {
    pub async fn search_anime(&self, query: &str, page: Option<u32>, limit: Option<u32>) -> Result<AnimeSearchResponse> {
        let mut endpoint = format!("anime?q={}", urlencoding::encode(query));
        
        if let Some(page) = page {
            endpoint.push_str(&format!("&page={}", page));
        }
        if let Some(limit) = limit {
            endpoint.push_str(&format!("&limit={}", limit));
        }
        
        self.get(&endpoint).await
    }

    pub async fn search_anime_advanced(
        &self,
        query: Option<&str>,
        page: Option<u32>,
        limit: Option<u32>,
        anime_type: Option<&str>,
        score: Option<f64>,
        min_score: Option<f64>,
        max_score: Option<f64>,
        status: Option<&str>,
        rating: Option<&str>,
        sfw: Option<bool>,
        genres: Option<&str>,
        genres_exclude: Option<&str>,
        order_by: Option<&str>,
        sort: Option<&str>,
        letter: Option<&str>,
        producers: Option<&str>,
        start_date: Option<&str>,
        end_date: Option<&str>,
    ) -> Result<AnimeSearchResponse> {
        let mut params = Vec::new();
        
        if let Some(q) = query {
            params.push(format!("q={}", urlencoding::encode(q)));
        }
        if let Some(page) = page {
            params.push(format!("page={}", page));
        }
        if let Some(limit) = limit {
            params.push(format!("limit={}", limit));
        }
        if let Some(anime_type) = anime_type {
            params.push(format!("type={}", anime_type));
        }
        if let Some(score) = score {
            params.push(format!("score={}", score));
        }
        if let Some(min_score) = min_score {
            params.push(format!("min_score={}", min_score));
        }
        if let Some(max_score) = max_score {
            params.push(format!("max_score={}", max_score));
        }
        if let Some(status) = status {
            params.push(format!("status={}", status));
        }
        if let Some(rating) = rating {
            params.push(format!("rating={}", rating));
        }
        if let Some(sfw) = sfw {
            params.push(format!("sfw={}", sfw));
        }
        if let Some(genres) = genres {
            params.push(format!("genres={}", genres));
        }
        if let Some(genres_exclude) = genres_exclude {
            params.push(format!("genres_exclude={}", genres_exclude));
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
        if let Some(producers) = producers {
            params.push(format!("producers={}", producers));
        }
        if let Some(start_date) = start_date {
            params.push(format!("start_date={}", start_date));
        }
        if let Some(end_date) = end_date {
            params.push(format!("end_date={}", end_date));
        }
        
        let endpoint = if params.is_empty() {
            "anime".to_string()
        } else {
            format!("anime?{}", params.join("&"))
        };
        
        self.get(&endpoint).await
    }

    pub async fn search_manga(&self, query: &str, page: Option<u32>, limit: Option<u32>) -> Result<MangaSearchResponse> {
        let mut endpoint = format!("manga?q={}", urlencoding::encode(query));
        
        if let Some(page) = page {
            endpoint.push_str(&format!("&page={}", page));
        }
        if let Some(limit) = limit {
            endpoint.push_str(&format!("&limit={}", limit));
        }
        
        self.get(&endpoint).await
    }

    pub async fn search_manga_advanced(
        &self,
        query: Option<&str>,
        page: Option<u32>,
        limit: Option<u32>,
        manga_type: Option<&str>,
        score: Option<f64>,
        min_score: Option<f64>,
        max_score: Option<f64>,
        status: Option<&str>,
        sfw: Option<bool>,
        genres: Option<&str>,
        genres_exclude: Option<&str>,
        order_by: Option<&str>,
        sort: Option<&str>,
        letter: Option<&str>,
        magazines: Option<&str>,
        start_date: Option<&str>,
        end_date: Option<&str>,
    ) -> Result<MangaSearchResponse> {
        let mut params = Vec::new();
        
        if let Some(q) = query {
            params.push(format!("q={}", urlencoding::encode(q)));
        }
        if let Some(page) = page {
            params.push(format!("page={}", page));
        }
        if let Some(limit) = limit {
            params.push(format!("limit={}", limit));
        }
        if let Some(manga_type) = manga_type {
            params.push(format!("type={}", manga_type));
        }
        if let Some(score) = score {
            params.push(format!("score={}", score));
        }
        if let Some(min_score) = min_score {
            params.push(format!("min_score={}", min_score));
        }
        if let Some(max_score) = max_score {
            params.push(format!("max_score={}", max_score));
        }
        if let Some(status) = status {
            params.push(format!("status={}", status));
        }
        if let Some(sfw) = sfw {
            params.push(format!("sfw={}", sfw));
        }
        if let Some(genres) = genres {
            params.push(format!("genres={}", genres));
        }
        if let Some(genres_exclude) = genres_exclude {
            params.push(format!("genres_exclude={}", genres_exclude));
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
        if let Some(magazines) = magazines {
            params.push(format!("magazines={}", magazines));
        }
        if let Some(start_date) = start_date {
            params.push(format!("start_date={}", start_date));
        }
        if let Some(end_date) = end_date {
            params.push(format!("end_date={}", end_date));
        }
        
        let endpoint = if params.is_empty() {
            "manga".to_string()
        } else {
            format!("manga?{}", params.join("&"))
        };
        
        self.get(&endpoint).await
    }

    pub async fn search_characters(&self, query: &str, page: Option<u32>, limit: Option<u32>) -> Result<CharacterSearchResponse> {
        let mut endpoint = format!("characters?q={}", urlencoding::encode(query));
        
        if let Some(page) = page {
            endpoint.push_str(&format!("&page={}", page));
        }
        if let Some(limit) = limit {
            endpoint.push_str(&format!("&limit={}", limit));
        }
        
        self.get(&endpoint).await
    }

    pub async fn search_people(&self, query: &str, page: Option<u32>, limit: Option<u32>) -> Result<PersonSearchResponse> {
        let mut endpoint = format!("people?q={}", urlencoding::encode(query));
        
        if let Some(page) = page {
            endpoint.push_str(&format!("&page={}", page));
        }
        if let Some(limit) = limit {
            endpoint.push_str(&format!("&limit={}", limit));
        }
        
        self.get(&endpoint).await
    }

    pub async fn search_users(&self, query: &str, page: Option<u32>, limit: Option<u32>) -> Result<UserSearchResponse> {
        let mut endpoint = format!("users?q={}", urlencoding::encode(query));
        
        if let Some(page) = page {
            endpoint.push_str(&format!("&page={}", page));
        }
        if let Some(limit) = limit {
            endpoint.push_str(&format!("&limit={}", limit));
        }
        
        self.get(&endpoint).await
    }

    pub async fn search_clubs(&self, query: &str, page: Option<u32>, limit: Option<u32>, category: Option<&str>, club_type: Option<&str>) -> Result<ClubSearchResponse> {
        let mut endpoint = format!("clubs?q={}", urlencoding::encode(query));
        
        if let Some(page) = page {
            endpoint.push_str(&format!("&page={}", page));
        }
        if let Some(limit) = limit {
            endpoint.push_str(&format!("&limit={}", limit));
        }
        if let Some(category) = category {
            endpoint.push_str(&format!("&category={}", category));
        }
        if let Some(club_type) = club_type {
            endpoint.push_str(&format!("&type={}", club_type));
        }
        
        self.get(&endpoint).await
    }
} 