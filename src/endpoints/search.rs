use crate::client::JikanClient;
use crate::error::Result;
use crate::models::search::*;

#[derive(Default)]
pub struct AnimeSearchParams {
    pub query: Option<String>,
    pub page: Option<u32>,
    pub limit: Option<u32>,
    pub anime_type: Option<String>,
    pub score: Option<f64>,
    pub min_score: Option<f64>,
    pub max_score: Option<f64>,
    pub status: Option<String>,
    pub rating: Option<String>,
    pub sfw: Option<bool>,
    pub genres: Option<String>,
    pub genres_exclude: Option<String>,
    pub order_by: Option<String>,
    pub sort: Option<String>,
    pub letter: Option<String>,
    pub producers: Option<String>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
}

#[derive(Default)]
pub struct MangaSearchParams {
    pub query: Option<String>,
    pub page: Option<u32>,
    pub limit: Option<u32>,
    pub manga_type: Option<String>,
    pub score: Option<f64>,
    pub min_score: Option<f64>,
    pub max_score: Option<f64>,
    pub status: Option<String>,
    pub sfw: Option<bool>,
    pub genres: Option<String>,
    pub genres_exclude: Option<String>,
    pub order_by: Option<String>,
    pub sort: Option<String>,
    pub letter: Option<String>,
    pub magazines: Option<String>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
}

impl JikanClient {
    pub async fn search_anime(
        &self,
        query: &str,
        page: Option<u32>,
        limit: Option<u32>,
    ) -> Result<AnimeSearchResponse> {
        let mut endpoint = format!("anime?q={}", urlencoding::encode(query));

        if let Some(page) = page {
            endpoint.push_str(&format!("&page={page}"));
        }
        if let Some(limit) = limit {
            endpoint.push_str(&format!("&limit={limit}"));
        }

        self.get(&endpoint).await
    }

    pub async fn search_anime_advanced(
        &self,
        params: AnimeSearchParams,
    ) -> Result<AnimeSearchResponse> {
        let mut query_params = Vec::new();

        if let Some(q) = &params.query {
            query_params.push(format!("q={}", urlencoding::encode(q)));
        }
        if let Some(page) = params.page {
            query_params.push(format!("page={page}"));
        }
        if let Some(limit) = params.limit {
            query_params.push(format!("limit={limit}"));
        }
        if let Some(anime_type) = &params.anime_type {
            query_params.push(format!("type={anime_type}"));
        }
        if let Some(score) = params.score {
            query_params.push(format!("score={score}"));
        }
        if let Some(min_score) = params.min_score {
            query_params.push(format!("min_score={min_score}"));
        }
        if let Some(max_score) = params.max_score {
            query_params.push(format!("max_score={max_score}"));
        }
        if let Some(status) = &params.status {
            query_params.push(format!("status={status}"));
        }
        if let Some(rating) = &params.rating {
            query_params.push(format!("rating={rating}"));
        }
        if let Some(sfw) = params.sfw {
            query_params.push(format!("sfw={sfw}"));
        }
        if let Some(genres) = &params.genres {
            query_params.push(format!("genres={genres}"));
        }
        if let Some(genres_exclude) = &params.genres_exclude {
            query_params.push(format!("genres_exclude={genres_exclude}"));
        }
        if let Some(order_by) = &params.order_by {
            query_params.push(format!("order_by={order_by}"));
        }
        if let Some(sort) = &params.sort {
            query_params.push(format!("sort={sort}"));
        }
        if let Some(letter) = &params.letter {
            query_params.push(format!("letter={letter}"));
        }
        if let Some(producers) = &params.producers {
            query_params.push(format!("producers={producers}"));
        }
        if let Some(start_date) = &params.start_date {
            query_params.push(format!("start_date={start_date}"));
        }
        if let Some(end_date) = &params.end_date {
            query_params.push(format!("end_date={end_date}"));
        }

        let endpoint = if query_params.is_empty() {
            "anime".to_string()
        } else {
            format!("anime?{}", query_params.join("&"))
        };

        self.get(&endpoint).await
    }

    pub async fn search_manga(
        &self,
        query: &str,
        page: Option<u32>,
        limit: Option<u32>,
    ) -> Result<MangaSearchResponse> {
        let mut endpoint = format!("manga?q={}", urlencoding::encode(query));

        if let Some(page) = page {
            endpoint.push_str(&format!("&page={page}"));
        }
        if let Some(limit) = limit {
            endpoint.push_str(&format!("&limit={limit}"));
        }

        self.get(&endpoint).await
    }

    pub async fn search_manga_advanced(
        &self,
        params: MangaSearchParams,
    ) -> Result<MangaSearchResponse> {
        let mut query_params = Vec::new();

        if let Some(q) = &params.query {
            query_params.push(format!("q={}", urlencoding::encode(q)));
        }
        if let Some(page) = params.page {
            query_params.push(format!("page={page}"));
        }
        if let Some(limit) = params.limit {
            query_params.push(format!("limit={limit}"));
        }
        if let Some(manga_type) = &params.manga_type {
            query_params.push(format!("type={manga_type}"));
        }
        if let Some(score) = params.score {
            query_params.push(format!("score={score}"));
        }
        if let Some(min_score) = params.min_score {
            query_params.push(format!("min_score={min_score}"));
        }
        if let Some(max_score) = params.max_score {
            query_params.push(format!("max_score={max_score}"));
        }
        if let Some(status) = &params.status {
            query_params.push(format!("status={status}"));
        }
        if let Some(sfw) = params.sfw {
            query_params.push(format!("sfw={sfw}"));
        }
        if let Some(genres) = &params.genres {
            query_params.push(format!("genres={genres}"));
        }
        if let Some(genres_exclude) = &params.genres_exclude {
            query_params.push(format!("genres_exclude={genres_exclude}"));
        }
        if let Some(order_by) = &params.order_by {
            query_params.push(format!("order_by={order_by}"));
        }
        if let Some(sort) = &params.sort {
            query_params.push(format!("sort={sort}"));
        }
        if let Some(letter) = &params.letter {
            query_params.push(format!("letter={letter}"));
        }
        if let Some(magazines) = &params.magazines {
            query_params.push(format!("magazines={magazines}"));
        }
        if let Some(start_date) = &params.start_date {
            query_params.push(format!("start_date={start_date}"));
        }
        if let Some(end_date) = &params.end_date {
            query_params.push(format!("end_date={end_date}"));
        }

        let endpoint = if query_params.is_empty() {
            "manga".to_string()
        } else {
            format!("manga?{}", query_params.join("&"))
        };

        self.get(&endpoint).await
    }

    pub async fn search_characters(
        &self,
        query: &str,
        page: Option<u32>,
        limit: Option<u32>,
    ) -> Result<CharacterSearchResponse> {
        let mut endpoint = format!("characters?q={}", urlencoding::encode(query));

        if let Some(page) = page {
            endpoint.push_str(&format!("&page={page}"));
        }
        if let Some(limit) = limit {
            endpoint.push_str(&format!("&limit={limit}"));
        }

        self.get(&endpoint).await
    }

    pub async fn search_people(
        &self,
        query: &str,
        page: Option<u32>,
        limit: Option<u32>,
    ) -> Result<PersonSearchResponse> {
        let mut endpoint = format!("people?q={}", urlencoding::encode(query));

        if let Some(page) = page {
            endpoint.push_str(&format!("&page={page}"));
        }
        if let Some(limit) = limit {
            endpoint.push_str(&format!("&limit={limit}"));
        }

        self.get(&endpoint).await
    }

    pub async fn search_users(
        &self,
        query: &str,
        page: Option<u32>,
        limit: Option<u32>,
    ) -> Result<UserSearchResponse> {
        let mut endpoint = format!("users?q={}", urlencoding::encode(query));

        if let Some(page) = page {
            endpoint.push_str(&format!("&page={page}"));
        }
        if let Some(limit) = limit {
            endpoint.push_str(&format!("&limit={limit}"));
        }

        self.get(&endpoint).await
    }

    pub async fn search_clubs(
        &self,
        query: &str,
        page: Option<u32>,
        limit: Option<u32>,
        category: Option<&str>,
        club_type: Option<&str>,
    ) -> Result<ClubSearchResponse> {
        let mut endpoint = format!("clubs?q={}", urlencoding::encode(query));

        if let Some(page) = page {
            endpoint.push_str(&format!("&page={page}"));
        }
        if let Some(limit) = limit {
            endpoint.push_str(&format!("&limit={limit}"));
        }
        if let Some(category) = category {
            endpoint.push_str(&format!("&category={category}"));
        }
        if let Some(club_type) = club_type {
            endpoint.push_str(&format!("&type={club_type}"));
        }

        self.get(&endpoint).await
    }
}
