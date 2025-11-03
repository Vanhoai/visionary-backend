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

    pub fn to_string(&self) -> String {
        match self {
            AuthProvider::Password => "PASSWORD".to_string(),
            AuthProvider::Google => "GOOGLE".to_string(),
            AuthProvider::Github => "GITHUB".to_string(),
        }
    }
}
