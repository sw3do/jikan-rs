use serde::{Deserialize, Serialize};
use crate::models::common::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Producer {
    pub mal_id: u32,
    pub url: String,
    pub titles: Vec<Title>,
    pub images: Images,
    pub favorites: u32,
    pub count: u32,
    pub established: Option<String>,
    pub about: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProducerResponse {
    pub data: Producer,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProducersResponse {
    pub data: Vec<Producer>,
    pub pagination: Pagination,
} 