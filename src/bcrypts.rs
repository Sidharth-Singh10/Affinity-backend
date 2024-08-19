use axum::Error;
use bcrypt::{DEFAULT_COST, hash, verify};

pub fn hash_password(password: &str) -> Result<String, Error> {
    let hashed = hash(password, DEFAULT_COST);

    match hashed {
        Ok(hash) => Ok(hash),             
        Err(e) => Err(Error::new(e)),     
    }
}

pub fn verify_password(password: &str, hashed_password:&str) ->bool{

    let valid = verify(password, hashed_password);

    match valid{
        Ok(valid) => valid,
        Err(e) => {
            eprintln!("Error verifying password: {}", e);
            false
        }

    }
}