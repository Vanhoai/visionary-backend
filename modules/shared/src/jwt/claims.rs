use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String, // Subject (Account ID)
    pub jit: String, // JWT ID
    pub exp: i64,
    pub iat: i64,
}
