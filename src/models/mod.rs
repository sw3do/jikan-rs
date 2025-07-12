pub mod anime;
pub mod character;
pub mod club;
pub mod common;
pub mod genre;
pub mod magazine;
pub mod manga;
pub mod person;
pub mod producer;
pub mod schedule;
pub mod search;
pub mod seasonal;
pub mod top;
pub mod user;
pub mod watch;

pub use anime::{
    Anime, AnimeCharactersResponse, AnimeEpisodeResponse, AnimeEpisodesResponse,
    AnimeExternalResponse, AnimeForumResponse, AnimeFullResponse, AnimeMoreInfoResponse,
    AnimeNewsResponse, AnimePicturesResponse, AnimeRecommendationsResponse, AnimeRelationsResponse,
    AnimeResponse, AnimeReviewsResponse, AnimeStaffResponse, AnimeStatisticsResponse,
    AnimeStreamingResponse, AnimeThemesResponse, AnimeUserUpdatesResponse,
    AnimeVideosEpisodesResponse, AnimeVideosResponse,
};

pub use manga::{
    Manga, MangaCharactersResponse, MangaExternalResponse, MangaForumResponse, MangaFullResponse,
    MangaMoreInfoResponse, MangaNewsResponse, MangaPicturesResponse, MangaRecommendationsResponse,
    MangaRelationsResponse, MangaResponse, MangaReviewsResponse, MangaStatisticsResponse,
    MangaUserUpdatesResponse,
};

pub use character::{
    Character as CharacterModel, CharacterAnimeResponse, CharacterFullResponse,
    CharacterMangaResponse, CharacterPicturesResponse, CharacterResponse,
    CharacterVoiceActorsResponse,
};

pub use person::{
    Person as PersonModel, PersonAnimeResponse, PersonFullResponse, PersonMangaResponse,
    PersonPicturesResponse, PersonResponse, PersonVoiceActingResponse,
};

pub use common::{
    Author, Broadcast, DateDetail, DateProp, DateRange, Demographic, Genre, Image, Images,
    Licensor, MalUrl, NamedResource, Pagination, PaginationItems, Serialization, Studio, Theme,
    Title, Trailer, TrailerImages,
};

pub use search::{
    AnimeSearchResponse, Character as SearchCharacter, CharacterSearchResponse, Club as SearchClub,
    ClubSearchResponse, MangaSearchResponse, Person as SearchPerson, PersonSearchResponse,
    UserProfile as SearchUserProfile, UserSearchResponse,
};

pub use genre::*;
pub use schedule::*;
pub use seasonal::*;
pub use top::*;

pub use user::{
    UserAnimeStatistics, UserMangaStatistics, UserProfile as UserProfileModel, UserProfileResponse,
    UserStatistics, UserStatisticsResponse,
};

pub use club::{
    Club as ClubModel, ClubMember, ClubMembersResponse, ClubRelation, ClubRelationsResponse,
    ClubResponse, ClubStaff, ClubStaffResponse,
};
pub use magazine::*;
pub use producer::{Producer as ProducerModel, ProducerResponse, ProducersResponse};
pub use watch::*;
