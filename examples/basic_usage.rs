use jikan_rs::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    let client = JikanClient::new();

    println!("Jikan API Rust Library Example Usage");
    println!("====================================");

    match client.get_anime(1).await {
        Ok(response) => {
            let anime = response.data;
            println!("Anime: {}", anime.title);
            println!("Score: {:?}", anime.score);
            println!("Status: {:?}", anime.status);
            println!("Episodes: {:?}", anime.episodes);
        }
        Err(e) => println!("Error: {e:?}"),
    }

    match client.search_anime("Naruto", Some(1), Some(5)).await {
        Ok(response) => {
            println!("\nNaruto search results:");
            for anime in response.data.iter().take(3) {
                println!("- {} (ID: {})", anime.title, anime.mal_id);
            }
        }
        Err(e) => println!("Search error: {e:?}"),
    }

    match client.get_top_anime(Some(1), Some(5), None, None).await {
        Ok(response) => {
            println!("\nTop anime list:");
            for (i, anime) in response.data.iter().enumerate() {
                println!("{}. {} - Score: {:?}", i + 1, anime.title, anime.score);
            }
        }
        Err(e) => println!("Top anime error: {e:?}"),
    }

    match client
        .get_season_now(Some(1), Some(5), None, None, None, None)
        .await
    {
        Ok(response) => {
            println!("\nCurrently airing anime:");
            for anime in response.data.iter().take(3) {
                println!("- {}", anime.title);
            }
        }
        Err(e) => println!("Season error: {e:?}"),
    }

    Ok(())
}
