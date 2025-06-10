use crate::client::JikanClient;
use crate::error::Result;
use crate::models::person::*;

impl JikanClient {
    pub async fn get_person(&self, id: u32) -> Result<PersonResponse> {
        let endpoint = format!("people/{}", id);
        self.get(&endpoint).await
    }

    pub async fn get_person_full(&self, id: u32) -> Result<PersonFullResponse> {
        let endpoint = format!("people/{}/full", id);
        self.get(&endpoint).await
    }

    pub async fn get_person_anime(&self, id: u32) -> Result<PersonAnimeResponse> {
        let endpoint = format!("people/{}/anime", id);
        self.get(&endpoint).await
    }

    pub async fn get_person_manga(&self, id: u32) -> Result<PersonMangaResponse> {
        let endpoint = format!("people/{}/manga", id);
        self.get(&endpoint).await
    }

    pub async fn get_person_voices(&self, id: u32) -> Result<PersonVoiceActingResponse> {
        let endpoint = format!("people/{}/voices", id);
        self.get(&endpoint).await
    }

    pub async fn get_person_pictures(&self, id: u32) -> Result<PersonPicturesResponse> {
        let endpoint = format!("people/{}/pictures", id);
        self.get(&endpoint).await
    }
} 