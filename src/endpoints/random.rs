use crate::client::JikanClient;
use crate::error::Result;
use crate::models::anime::AnimeResponse;
use crate::models::character::CharacterResponse;
use crate::models::manga::MangaResponse;
use crate::models::person::PersonResponse;
use crate::models::user::UserProfileResponse;

impl JikanClient {
    pub async fn get_random_anime(&self) -> Result<AnimeResponse> {
        let endpoint = "random/anime";
        self.get(endpoint).await
    }

    pub async fn get_random_manga(&self) -> Result<MangaResponse> {
        let endpoint = "random/manga";
        self.get(endpoint).await
    }

    pub async fn get_random_character(&self) -> Result<CharacterResponse> {
        let endpoint = "random/characters";
        self.get(endpoint).await
    }

    pub async fn get_random_person(&self) -> Result<PersonResponse> {
        let endpoint = "random/people";
        self.get(endpoint).await
    }

    pub async fn get_random_user(&self) -> Result<UserProfileResponse> {
        let endpoint = "random/users";
        self.get(endpoint).await
    }
}
