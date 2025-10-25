use entity::{user_details, users};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct SignUpInfo {
    pub username: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub gender: String,
    pub age: i32,
    pub location: Option<String>,
    pub openness: Option<String>,
    pub interests: Option<String>,
    pub exp_qual: Option<String>,
    pub relation_type: Option<String>,
    pub social_habits: Option<String>,
    pub past_relations: Option<String>,
    pub values: Option<String>,
    pub style: Option<String>,
    pub traits: Option<String>,
    pub commitment: Option<String>,
    pub resolution: Option<String>,
    pub score: f32,
    pub image_url: Option<String>,
    pub bio: Option<String>,
}

#[derive(Deserialize)]
pub struct LoginInfo {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct GetUserInfo {
    pub id: i32,
}
#[derive(Deserialize)]
pub struct UpdateScoreInfo {
    pub user_id: i32,
    pub score: f32,
}
#[derive(Serialize)]
pub struct LoginResponse {
    pub user: users::Model,
    pub user_details: Option<user_details::Model>,
    pub token: String,
}
#[derive(Deserialize)]
pub struct MatchListInfo {
    pub female_id: i32,
    pub male_id: i32,
}

#[derive(Deserialize)]
pub struct CharacterDetails {
    pub user_id: i32,
    pub interests: Option<String>,
    pub values: Option<String>,
    pub style: Option<String>,
    pub traits: Option<String>,
    pub commitment: Option<String>,
    pub resolution: Option<String>,
}

#[derive(Deserialize)]
pub struct GirlBoyInfoById {
    pub id: String,
}

#[derive(Deserialize)]
pub struct UpdateBioPayload {
    pub bio: String,
}

#[derive(Deserialize, Serialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    // pub email: String,
}
#[derive(Deserialize, Serialize)]

pub struct GameSession {
    pub id: i32,
    pub score: f32,
}
#[derive(Serialize)]

pub struct BoyScoreInfo {
    pub match_id: i32,
    pub score: f32,
    pub male_id: i32,
    pub game_id: i32,
}
