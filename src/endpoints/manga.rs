use crate::client::JikanClient;
use crate::error::Result;
use crate::models::manga::*;

impl JikanClient {
    pub async fn get_manga(&self, id: u32) -> Result<MangaResponse> {
        let endpoint = format!("manga/{id}");
        self.get(&endpoint).await
    }

    pub async fn get_manga_full(&self, id: u32) -> Result<MangaFullResponse> {
        let endpoint = format!("manga/{id}/full");
        self.get(&endpoint).await
    }

    pub async fn get_manga_characters(&self, id: u32) -> Result<MangaCharactersResponse> {
        let endpoint = format!("manga/{id}/characters");
        self.get(&endpoint).await
    }

    pub async fn get_manga_news(&self, id: u32, page: Option<u32>) -> Result<MangaNewsResponse> {
        let mut endpoint = format!("manga/{id}/news");
        if let Some(page) = page {
            endpoint.push_str(&format!("?page={page}"));
        }
        self.get(&endpoint).await
    }

    pub async fn get_manga_forum(
        &self,
        id: u32,
        filter: Option<&str>,
    ) -> Result<MangaForumResponse> {
        let mut endpoint = format!("manga/{id}/forum");
        if let Some(filter) = filter {
            endpoint.push_str(&format!("?filter={filter}"));
        }
        self.get(&endpoint).await
    }

    pub async fn get_manga_pictures(&self, id: u32) -> Result<MangaPicturesResponse> {
        let endpoint = format!("manga/{id}/pictures");
        self.get(&endpoint).await
    }

    pub async fn get_manga_statistics(&self, id: u32) -> Result<MangaStatisticsResponse> {
        let endpoint = format!("manga/{id}/statistics");
        self.get(&endpoint).await
    }

    pub async fn get_manga_more_info(&self, id: u32) -> Result<MangaMoreInfoResponse> {
        let endpoint = format!("manga/{id}/moreinfo");
        self.get(&endpoint).await
    }

    pub async fn get_manga_recommendations(&self, id: u32) -> Result<MangaRecommendationsResponse> {
        let endpoint = format!("manga/{id}/recommendations");
        self.get(&endpoint).await
    }

    pub async fn get_manga_user_updates(
        &self,
        id: u32,
        page: Option<u32>,
    ) -> Result<MangaUserUpdatesResponse> {
        let mut endpoint = format!("manga/{id}/userupdates");
        if let Some(page) = page {
            endpoint.push_str(&format!("?page={page}"));
        }
        self.get(&endpoint).await
    }

    pub async fn get_manga_reviews(
        &self,
        id: u32,
        page: Option<u32>,
        preliminary: Option<bool>,
        spoiler: Option<bool>,
    ) -> Result<MangaReviewsResponse> {
        let mut endpoint = format!("manga/{id}/reviews");
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

    pub async fn get_manga_relations(&self, id: u32) -> Result<MangaRelationsResponse> {
        let endpoint = format!("manga/{id}/relations");
        self.get(&endpoint).await
    }

    pub async fn get_manga_external(&self, id: u32) -> Result<MangaExternalResponse> {
        let endpoint = format!("manga/{id}/external");
        self.get(&endpoint).await
    }
}
