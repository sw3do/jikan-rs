use jikan_rs::prelude::*;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let client = JikanClient::new();

    println!("🎌 Jikan API Rust Library - Comprehensive Example");
    println!("=================================================");

    println!("\n📺 Anime Information:");
    match client.get_anime(1).await {
        Ok(response) => {
            let anime = response.data;
            println!("  Title: {}", anime.title);
            println!("  Score: {:?}", anime.score);
            println!("  Status: {:?}", anime.status);
            println!("  Episodes: {:?}", anime.episodes);
        }
        Err(e) => println!("  Error: {e:?}"),
    }

    sleep(Duration::from_millis(500)).await;

    println!("\n📚 Manga Information:");
    match client.get_manga(1).await {
        Ok(response) => {
            let manga = response.data;
            println!("  Title: {}", manga.title);
            println!("  Score: {:?}", manga.score);
            println!("  Status: {:?}", manga.status);
        }
        Err(e) => println!("  Error: {e:?}"),
    }

    sleep(Duration::from_millis(500)).await;

    println!("\n👤 Character Information:");
    match client.get_character(1).await {
        Ok(response) => {
            let character = response.data;
            println!("  Name: {}", character.name);
            println!("  Favorites: {}", character.favorites);
        }
        Err(e) => println!("  Error: {e:?}"),
    }

    sleep(Duration::from_millis(500)).await;

    println!("\n🎭 Person Information:");
    match client.get_person(1).await {
        Ok(response) => {
            let person = response.data;
            println!("  Name: {}", person.name);
            println!("  Favorites: {}", person.favorites);
        }
        Err(e) => println!("  Error: {e:?}"),
    }

    sleep(Duration::from_millis(500)).await;

    println!("\n🔍 Search Results:");
    match client.search_anime("Naruto", Some(1), Some(3)).await {
        Ok(response) => {
            for anime in response.data {
                println!("  - {} (ID: {})", anime.title, anime.mal_id);
            }
        }
        Err(e) => println!("  Error: {e:?}"),
    }

    sleep(Duration::from_millis(500)).await;

    println!("\n🏆 Top Anime:");
    match client.get_top_anime(Some(1), Some(3), None, None).await {
        Ok(response) => {
            for (i, anime) in response.data.iter().enumerate() {
                println!("  {}. {} - Score: {:?}", i + 1, anime.title, anime.score);
            }
        }
        Err(e) => println!("  Error: {e:?}"),
    }

    sleep(Duration::from_millis(500)).await;

    println!("\n🌸 Current Season:");
    match client
        .get_season_now(Some(1), Some(3), None, None, None, None)
        .await
    {
        Ok(response) => {
            for anime in response.data {
                println!("  - {}", anime.title);
            }
        }
        Err(e) => println!("  Error: {e:?}"),
    }

    sleep(Duration::from_millis(500)).await;

    println!("\n🎲 Random Anime:");
    match client.get_random_anime().await {
        Ok(response) => {
            let anime = response.data;
            println!("  Title: {}", anime.title);
            println!("  Score: {:?}", anime.score);
        }
        Err(e) => println!("  Error: {e:?}"),
    }

    sleep(Duration::from_millis(500)).await;

    println!("\n📅 Schedule:");
    match client.get_schedules(None, None, None, None, None).await {
        Ok(response) => {
            for anime in response.data.iter().take(3) {
                println!("  - {}", anime.title);
            }
        }
        Err(e) => println!("  Error: {e:?}"),
    }

    sleep(Duration::from_millis(500)).await;

    println!("\n🎬 Recent Episodes:");
    match client.get_watch_recent_episodes(Some(1)).await {
        Ok(response) => {
            for episode in response.data.iter().take(3) {
                println!(
                    "  - {}",
                    episode.title.as_ref().unwrap_or(&"No title".to_string())
                );
            }
        }
        Err(e) => println!("  Error: {e:?}"),
    }

    println!("\n✅ All tests completed!");
    Ok(())
}
