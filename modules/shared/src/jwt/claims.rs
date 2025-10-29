use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    aud: String, // Optional. Audience
    exp: usize,  // Required. Expiration time (as UTC timestamp)
    iat: usize,  // Optional. Issued at (as UTC timestamp)
    iss: String, // Optional. Issuer
    nbf: usize,  // Optional. Not Before (as UTC timestamp)
    sub: String, // Optional. Subject (whom token refers to)
}
