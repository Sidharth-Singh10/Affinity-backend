use serde::{Deserialize, Serialize};

#[derive(Deserialize,Serialize)]
pub struct SignUpInfo {
    pub username: String,
    pub password: String,
    pub first_name:String,
    pub last_name:String,
    pub email: String,         
    pub gender: String,
    pub age: i32,      
    pub location: Option<String>,   
    pub openness: Option<String>,   
    pub interest: Option<String>,  
    pub exp_qual: Option<String>,   
    pub relation_type: Option<String>, 
    pub social_habits: Option<String>, 
    pub past_relations: Option<String>,
    pub values: Option<String>,
    pub style: Option<String>,
    pub traits: Option<String>,
    pub commitment: Option<String>,
    pub resolution: Option<String>, 
    pub image_url: Option<String>,
}

#[derive(Deserialize)]
pub struct LoginInfo {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
}

#[derive(Deserialize, Serialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}


