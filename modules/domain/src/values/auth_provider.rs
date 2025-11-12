use std::fmt::Display;

// shared modules
use shared::models::failure::Failure;

pub enum AuthProvider {
    Password,
    Google,
    Github,
}

impl AuthProvider {
    pub fn from_string(s: &str) -> Result<Self, Failure> {
        match s {
            "PASSWORD" => Ok(AuthProvider::Password),
            "GOOGLE" => Ok(AuthProvider::Google),
            "GITHUB" => Ok(AuthProvider::Github),
            _ => Err(Failure::BadRequest(format!("Invalid auth provider: {}", s))),
        }
    }
}

impl Display for AuthProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            AuthProvider::Password => "PASSWORD",
            AuthProvider::Google => "GOOGLE",
            AuthProvider::Github => "GITHUB",
        };
        write!(f, "{}", s)
    }
}
