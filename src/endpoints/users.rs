use crate::client::JikanClient;
use crate::error::Result;
use crate::models::user::*;

impl JikanClient {
    pub async fn get_user_profile(&self, username: &str) -> Result<UserProfileResponse> {
        let endpoint = format!("users/{username}");
        self.get(&endpoint).await
    }

    pub async fn get_user_statistics(&self, username: &str) -> Result<UserStatisticsResponse> {
        let endpoint = format!("users/{username}/statistics");
        self.get(&endpoint).await
    }
}
