use dotenv::dotenv;
use lazy_static::lazy_static;
use std::env;

lazy_static! {
    pub static ref DATABASE_URL: String = {
        dotenv().ok(); // loading env vars from the file
        env::var("DATABASE_URL").expect("DATABASE_URL must be set")  
};
}
