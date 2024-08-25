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
    pub score:i32,
    pub image_url: Option<String>,
}

#[derive(Deserialize)]
pub struct LoginInfo {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct GetUserInfo{
    pub email:String,
}
#[derive(Deserialize)]
pub struct UpadateScoreInfo{
    pub email:String,
    pub score: i32,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
}
#[derive(Deserialize)]
pub struct FriendListInfo{
    pub girl_email:String,
    pub boy_email:String,
}

#[derive(Deserialize)]
pub struct CharacterDetails{
    pub email:String,
    pub interests:Option<String>,
    pub values:Option<String>,
    pub style:Option<String>,
    pub traits:Option<String>,
    pub commitment:Option<String> ,
    pub resolution:Option<String>,
}

#[derive(Deserialize)]
pub struct GirlBoyInfo{
    pub email:String,
}
#[derive(Deserialize)]
pub struct GirlBoyInfoById{
    pub id:String,
}

#[derive(Deserialize, Serialize)]
pub struct Matched{
    pub boy_email: String,
    pub girl_email: String,
}

#[derive(Deserialize)]
pub struct ContestInfo
{
    pub id:String,
    pub contestscore:String,
}



#[derive(Deserialize, Serialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}


