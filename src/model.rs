use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
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
    pub fav_activ: Option<String>,  
    pub exp_qual: Option<String>,   
    pub relation_type: Option<String>, 
    pub social_habits: Option<String>, 
    pub comm_method: Option<String>,   
    pub past_relations: Option<String>, 
    pub image_url: Option<String>,
    pub score: i32,
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


