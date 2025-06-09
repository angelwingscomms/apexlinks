use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};

pub fn hash_password(password: &str) -> String {
    // Generate a salt
    let salt = SaltString::generate(&mut OsRng);
    
    // Create an Argon2 instance
    let argon2 = Argon2::default();
    
    // Hash password
    let password_hash = argon2.hash_password(password.as_bytes(), &salt)
        .expect("Failed to hash password")
        .to_string();
    
    password_hash
}

pub fn verify_password(hash: &str, password: &str) -> bool {
    // Parse hash
    match PasswordHash::new(hash) {
        Ok(parsed_hash) => {
            // Verify password against hash
            Argon2::default()
                .verify_password(password.as_bytes(), &parsed_hash)
                .is_ok()
        },
        Err(_) => false
    }
}