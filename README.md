# Jikan-rs


[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Build Status](https://github.com/sw3do/jikan-rs/workflows/CI/badge.svg)](https://github.com/sw3do/jikan-rs/actions)

A comprehensive Rust library for the [Jikan API](https://jikan.moe/) - an unofficial REST API for MyAnimeList. This library provides type-safe access to all Jikan API v4 endpoints with built-in rate limiting and error handling.

## ✨ Features

- ✅ **Rate Limiting** - Built-in rate limiting (3 requests per second, burst of 5)
- ✅ **Complete API Coverage** - All Jikan API v4 endpoints supported
- ✅ **Async/Await Support** - Full async/await compatibility with tokio
- ✅ **Type Safety** - Strongly typed responses with serde
- ✅ **Comprehensive Error Handling** - Detailed error types for better debugging
- ✅ **JSON Serialization** - Automatic JSON parsing with serde
- ✅ **URL Encoding** - Proper URL encoding for search queries
- ✅ **Random Endpoints** - Support for random anime, manga, characters
- ✅ **Watch Endpoints** - Recent and popular episodes/promos
- ✅ **Club Endpoints** - MyAnimeList club information

## 🚀 Quick Start

Add this to your `Cargo.toml`:

```toml
[dependencies]
jikan-rs = "0.1.0"
tokio = { version = "1.0", features = ["full"] }
```

## 📖 Basic Usage

```rust
use jikan_rs::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    let client = JikanClient::new();

    // Get anime information
    let anime = client.get_anime(1).await?;
    println!("Anime: {}", anime.data.title);

    // Search for anime
    let search_results = client.search_anime("Naruto", Some(1), Some(10)).await?;
    for anime in search_results.data {
        println!("- {}", anime.title);
    }

    // Get top anime
    let top_anime = client.get_top_anime(Some(1), Some(10), None, None).await?;
    for anime in top_anime.data {
        println!("- {} (Score: {:?})", anime.title, anime.score);
    }

    // Get random anime
    let random_anime = client.get_random_anime().await?;
    println!("Random anime: {}", random_anime.data.title);

    Ok(())
}
```

## 🎯 Supported Endpoints

### 📺 Anime
- `get_anime(id)` - Get anime by ID
- `get_anime_full(id)` - Get full anime details
- `get_anime_characters(id)` - Get anime characters
- `get_anime_staff(id)` - Get anime staff
- `get_anime_episodes(id, page)` - Get anime episodes
- `get_anime_episode(id, episode)` - Get specific episode
- `get_anime_news(id, page)` - Get anime news
- `get_anime_forum(id, filter)` - Get anime forum topics
- `get_anime_videos(id)` - Get anime videos
- `get_anime_videos_episodes(id, page)` - Get episode videos
- `get_anime_pictures(id)` - Get anime pictures
- `get_anime_statistics(id)` - Get anime statistics
- `get_anime_more_info(id)` - Get additional information
- `get_anime_recommendations(id)` - Get recommendations
- `get_anime_user_updates(id, page)` - Get user updates
- `get_anime_reviews(id, page, preliminary, spoiler)` - Get anime reviews
- `get_anime_relations(id)` - Get related anime
- `get_anime_themes(id)` - Get opening/ending themes
- `get_anime_external(id)` - Get external links
- `get_anime_streaming(id)` - Get streaming platforms

### 📚 Manga
- `get_manga(id)` - Get manga by ID
- `get_manga_full(id)` - Get full manga details
- `get_manga_characters(id)` - Get manga characters
- `get_manga_news(id, page)` - Get manga news
- `get_manga_forum(id, filter)` - Get manga forum topics
- `get_manga_pictures(id)` - Get manga pictures
- `get_manga_statistics(id)` - Get manga statistics
- `get_manga_more_info(id)` - Get additional information
- `get_manga_recommendations(id)` - Get recommendations
- `get_manga_user_updates(id, page)` - Get user updates
- `get_manga_reviews(id, page, preliminary, spoiler)` - Get manga reviews
- `get_manga_relations(id)` - Get related manga
- `get_manga_external(id)` - Get external links

### 👤 Characters
- `get_character(id)` - Get character information
- `get_character_full(id)` - Get full character details
- `get_character_anime(id)` - Get character's anime appearances
- `get_character_manga(id)` - Get character's manga appearances
- `get_character_voice_actors(id)` - Get voice actors
- `get_character_pictures(id)` - Get character pictures

### 🎭 People
- `get_person(id)` - Get person information
- `get_person_full(id)` - Get full person details
- `get_person_anime(id)` - Get person's anime work
- `get_person_manga(id)` - Get person's manga work
- `get_person_voice_acting(id)` - Get voice acting roles
- `get_person_pictures(id)` - Get person pictures

### 🔍 Search
- `search_anime(query, page, limit)` - Search anime
- `search_anime_advanced(...)` - Advanced anime search
- `search_manga(query, page, limit)` - Search manga
- `search_manga_advanced(...)` - Advanced manga search
- `search_characters(query, page, limit)` - Search characters
- `search_people(query, page, limit)` - Search people
- `search_users(query, page, limit)` - Search users

### 🏆 Top Lists
- `get_top_anime(page, limit, type, filter)` - Top anime
- `get_top_manga(page, limit, type, filter)` - Top manga
- `get_top_characters(page, limit)` - Top characters
- `get_top_people(page, limit)` - Top people
- `get_recent_anime_reviews(page)` - Recent anime reviews
- `get_recent_manga_reviews(page)` - Recent manga reviews

### 🌸 Seasonal Anime
- `get_season_now(...)` - Currently airing anime
- `get_season(year, season, ...)` - Anime from specific season
- `get_season_upcoming(...)` - Upcoming anime
- `get_seasons_archive()` - Season archive

### 🎲 Random
- `get_random_anime()` - Random anime
- `get_random_manga()` - Random manga
- `get_random_character()` - Random character
- `get_random_person()` - Random person
- `get_random_user()` - Random user

### 📺 Watch
- `get_watch_recent_episodes(page)` - Recent episodes
- `get_watch_popular_episodes(page)` - Popular episodes
- `get_watch_recent_promos(page)` - Recent promotional videos
- `get_watch_popular_promos(page)` - Popular promotional videos

### 🏛️ Clubs
- `get_club(id)` - Club information
- `get_club_members(id, page)` - Club members
- `get_club_staff(id)` - Club staff
- `get_club_relations(id)` - Club relations

### 🔧 Other
- `get_schedules(...)` - Anime broadcast schedule
- `get_anime_genres(filter)` - Anime genres
- `get_manga_genres(filter)` - Manga genres
- `get_user_profile(username)` - User profile
- `get_producers(page, limit)` - Producers
- `get_producer(id)` - Producer information
- `get_magazines(page, limit)` - Magazines
- `get_magazine(id)` - Magazine information

## ⚡ Rate Limiting

The library automatically handles rate limiting (3 requests per second by default). You can customize this:

```rust
// Custom rate limit: 2 requests per second, burst of 3
let client = JikanClient::with_rate_limit(2, 3);
```

To disable rate limiting completely:

```toml
[dependencies]
jikan-rs = { version = "0.1.0", default-features = false }
```

## 🛠️ Error Handling

```rust
match client.get_anime(999999).await {
    Ok(response) => println!("Anime: {}", response.data.title),
    Err(JikanError::NotFound) => println!("Anime not found"),
    Err(JikanError::RateLimitExceeded) => println!("Rate limit exceeded"),
    Err(JikanError::ApiError { status, message }) => {
        println!("API Error {}: {}", status, message)
    },
    Err(e) => println!("Error: {:?}", e),
}
```

## 📋 Examples

Run the basic example:
```bash
cargo run --example basic_usage
```

Run the comprehensive example:
```bash
cargo run --example comprehensive_usage
```

## 🧪 Testing

```bash
cargo test
```

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- [Jikan API](https://jikan.moe/) for providing the excellent MyAnimeList API
- [MyAnimeList](https://myanimelist.net/) for being the ultimate anime and manga database
- The Rust community for amazing crates and tools


## 🔗 Links


- [Repository](https://github.com/sw3do/jikan-rs)
- [Jikan API Documentation](https://docs.api.jikan.moe/)

---

Made with ❤️ by [sw3do](https://github.com/sw3do) 