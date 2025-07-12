use crate::client::JikanClient;
use crate::error::Result;
use crate::models::anime::*;

impl JikanClient {
    pub async fn get_anime(&self, id: u32) -> Result<AnimeResponse> {
        let endpoint = format!("anime/{id}");
        self.get(&endpoint).await
    }

    pub async fn get_anime_full(&self, id: u32) -> Result<AnimeFullResponse> {
        let endpoint = format!("anime/{id}/full");
        self.get(&endpoint).await
    }

    pub async fn get_anime_characters(&self, id: u32) -> Result<AnimeCharactersResponse> {
        let endpoint = format!("anime/{id}/characters");
        self.get(&endpoint).await
    }

    pub async fn get_anime_staff(&self, id: u32) -> Result<AnimeStaffResponse> {
        let endpoint = format!("anime/{id}/staff");
        self.get(&endpoint).await
    }

    pub async fn get_anime_episodes(
        &self,
        id: u32,
        page: Option<u32>,
    ) -> Result<AnimeEpisodesResponse> {
        let mut endpoint = format!("anime/{id}/episodes");
        if let Some(page) = page {
            endpoint.push_str(&format!("?page={page}"));
        }
        self.get(&endpoint).await
    }

    pub async fn get_anime_episode(&self, id: u32, episode: u32) -> Result<AnimeEpisodeResponse> {
        let endpoint = format!("anime/{id}/episodes/{episode}");
        self.get(&endpoint).await
    }

    pub async fn get_anime_news(&self, id: u32, page: Option<u32>) -> Result<AnimeNewsResponse> {
        let mut endpoint = format!("anime/{id}/news");
        if let Some(page) = page {
            endpoint.push_str(&format!("?page={page}"));
        }
        self.get(&endpoint).await
    }

    pub async fn get_anime_forum(
        &self,
        id: u32,
        filter: Option<&str>,
    ) -> Result<AnimeForumResponse> {
        let mut endpoint = format!("anime/{id}/forum");
        if let Some(filter) = filter {
            endpoint.push_str(&format!("?filter={filter}"));
        }
        self.get(&endpoint).await
    }

    pub async fn get_anime_videos(&self, id: u32) -> Result<AnimeVideosResponse> {
        let endpoint = format!("anime/{id}/videos");
        self.get(&endpoint).await
    }

    pub async fn get_anime_videos_episodes(
        &self,
        id: u32,
        page: Option<u32>,
    ) -> Result<AnimeVideosEpisodesResponse> {
        let mut endpoint = format!("anime/{id}/videos/episodes");
        if let Some(page) = page {
            endpoint.push_str(&format!("?page={page}"));
        }
        self.get(&endpoint).await
    }

    pub async fn get_anime_pictures(&self, id: u32) -> Result<AnimePicturesResponse> {
        let endpoint = format!("anime/{id}/pictures");
        self.get(&endpoint).await
    }

    pub async fn get_anime_statistics(&self, id: u32) -> Result<AnimeStatisticsResponse> {
        let endpoint = format!("anime/{id}/statistics");
        self.get(&endpoint).await
    }

    pub async fn get_anime_more_info(&self, id: u32) -> Result<AnimeMoreInfoResponse> {
        let endpoint = format!("anime/{id}/moreinfo");
        self.get(&endpoint).await
    }

    pub async fn get_anime_recommendations(&self, id: u32) -> Result<AnimeRecommendationsResponse> {
        let endpoint = format!("anime/{id}/recommendations");
        self.get(&endpoint).await
    }

    pub async fn get_anime_user_updates(
        &self,
        id: u32,
        page: Option<u32>,
    ) -> Result<AnimeUserUpdatesResponse> {
        let mut endpoint = format!("anime/{id}/userupdates");
        if let Some(page) = page {
            endpoint.push_str(&format!("?page={page}"));
        }
        self.get(&endpoint).await
    }

    pub async fn get_anime_reviews(
        &self,
        id: u32,
        page: Option<u32>,
        preliminary: Option<bool>,
        spoiler: Option<bool>,
    ) -> Result<AnimeReviewsResponse> {
        let mut endpoint = format!("anime/{id}/reviews");
        let mut params = Vec::new();

        if let Some(page) = page {
            params.push(format!("page={page}"));
        }
        if let Some(preliminary) = preliminary {
            params.push(format!("preliminary={preliminary}"));
        }
        if let Some(spoiler) = spoiler {
            params.push(format!("spoiler={spoiler}"));
        }

        if !params.is_empty() {
            endpoint.push_str(&format!("?{}", params.join("&")));
        }

        self.get(&endpoint).await
    }

    pub async fn get_anime_relations(&self, id: u32) -> Result<AnimeRelationsResponse> {
        let endpoint = format!("anime/{id}/relations");
        self.get(&endpoint).await
    }

    pub async fn get_anime_themes(&self, id: u32) -> Result<AnimeThemesResponse> {
        let endpoint = format!("anime/{id}/themes");
        self.get(&endpoint).await
    }

    pub async fn get_anime_external(&self, id: u32) -> Result<AnimeExternalResponse> {
        let endpoint = format!("anime/{id}/external");
        self.get(&endpoint).await
    }

    pub async fn get_anime_streaming(&self, id: u32) -> Result<AnimeStreamingResponse> {
        let endpoint = format!("anime/{id}/streaming");
        self.get(&endpoint).await
    }
}
