use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString, Error},
    Argon2, Algorithm, Version,
};

use crate::config;

pub async fn hash(input: &String) -> Result<String, Error> {
    let salt = SaltString::generate(&mut OsRng);

    let argon2 = Argon2::new(
        Algorithm::Argon2id, 
        Version::V0x13,      
        config::ARGON2_PARAMS?,              
    );

    let password_hash = argon2.hash_password(input.as_bytes(), &salt)?.to_string();
    
    Ok(password_hash)
}
