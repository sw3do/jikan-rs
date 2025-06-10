use crate::client::JikanClient;
use crate::error::Result;
use crate::models::character::*;

impl JikanClient {
    pub async fn get_character(&self, id: u32) -> Result<CharacterResponse> {
        let endpoint = format!("characters/{}", id);
        self.get(&endpoint).await
    }

    pub async fn get_character_full(&self, id: u32) -> Result<CharacterFullResponse> {
        let endpoint = format!("characters/{}/full", id);
        self.get(&endpoint).await
    }

    pub async fn get_character_anime(&self, id: u32) -> Result<CharacterAnimeResponse> {
        let endpoint = format!("characters/{}/anime", id);
        self.get(&endpoint).await
    }

    pub async fn get_character_manga(&self, id: u32) -> Result<CharacterMangaResponse> {
        let endpoint = format!("characters/{}/manga", id);
        self.get(&endpoint).await
    }

    pub async fn get_character_voice_actors(&self, id: u32) -> Result<CharacterVoiceActorsResponse> {
        let endpoint = format!("characters/{}/voices", id);
        self.get(&endpoint).await
    }

    pub async fn get_character_pictures(&self, id: u32) -> Result<CharacterPicturesResponse> {
        let endpoint = format!("characters/{}/pictures", id);
        self.get(&endpoint).await
    }
} 