use crate::client::JikanClient;
use crate::error::Result;
use crate::models::genre::*;

impl JikanClient {
    pub async fn get_anime_genres(&self, filter: Option<&str>) -> Result<AnimeGenresResponse> {
        let endpoint = if let Some(filter) = filter {
            format!("genres/anime?filter={filter}")
        } else {
            "genres/anime".to_string()
        };

        self.get(&endpoint).await
    }

    pub async fn get_manga_genres(&self, filter: Option<&str>) -> Result<MangaGenresResponse> {
        let endpoint = if let Some(filter) = filter {
            format!("genres/manga?filter={filter}")
        } else {
            "genres/manga".to_string()
        };

        self.get(&endpoint).await
    }
}
