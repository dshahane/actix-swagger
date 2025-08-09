use actix_web_httpauth::extractors::basic::BasicAuth;
use actix_web::Error;

// This is a simple, hardcoded user store. In a real application,
// this would be a database or another secure storage.
const VALID_USERNAME: &str = "user";
const VALID_PASSWORD: &str = "password";

// Synchronous function to validate Basic authentication credentials.
pub fn validate_basic_auth(auth: &BasicAuth) -> Result<(), Error> {
    // Extract the username and password from the BasicAuth struct
    let username = auth.user_id();
    let password = auth.password().unwrap_or("");

    // Check if the credentials match our hardcoded values
    if username == VALID_USERNAME && password == VALID_PASSWORD {
        // If they match, return Ok.
        Ok(())
    } else {
        // If they don't match, return an error.
        Err(actix_web::error::ErrorUnauthorized("Invalid credentials"))
    }
}