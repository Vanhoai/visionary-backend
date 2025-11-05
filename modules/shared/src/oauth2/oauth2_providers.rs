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

    pub fn to_string(&self) -> String {
        match self {
            OAuth2Provider::Google => "GOOGLE".to_string(),
            OAuth2Provider::GitHub => "GITHUB".to_string(),
        }
    }
}