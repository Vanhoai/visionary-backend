use std::fmt::Display;

use crate::models::failure::Failure;

pub enum OAuth2Provider {
    Google,
    GitHub,
}

impl OAuth2Provider {
    pub fn from_string(s: &str) -> Result<Self, Failure> {
        match s.to_uppercase().as_str() {
            "GOOGLE" => Ok(OAuth2Provider::Google),
            "GITHUB" => Ok(OAuth2Provider::GitHub),
            _ => Err(Failure::BadRequest(format!("Invalid OAuth2 provider: {}", s))),
        }
    }
}

impl Display for OAuth2Provider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            OAuth2Provider::Google => "GOOGLE",
            OAuth2Provider::GitHub => "GITHUB",
        };
        write!(f, "{}", s)
    }
}
