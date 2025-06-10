use crate::client::JikanClient;
use crate::error::Result;
use crate::models::club::*;

impl JikanClient {
    pub async fn get_club(&self, id: u32) -> Result<ClubResponse> {
        let endpoint = format!("clubs/{}", id);
        self.get(&endpoint).await
    }

    pub async fn get_club_members(&self, id: u32, page: Option<u32>) -> Result<ClubMembersResponse> {
        let mut endpoint = format!("clubs/{}/members", id);
        if let Some(page) = page {
            endpoint.push_str(&format!("?page={}", page));
        }
        self.get(&endpoint).await
    }

    pub async fn get_club_staff(&self, id: u32) -> Result<ClubStaffResponse> {
        let endpoint = format!("clubs/{}/staff", id);
        self.get(&endpoint).await
    }

    pub async fn get_club_relations(&self, id: u32) -> Result<ClubRelationsResponse> {
        let endpoint = format!("clubs/{}/relations", id);
        self.get(&endpoint).await
    }
} 