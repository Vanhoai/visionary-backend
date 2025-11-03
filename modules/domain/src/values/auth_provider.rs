pub enum AuthProvider {
    Password,
    Google,
    Github,
}

impl AuthProvider {
    pub fn to_string(&self) -> String {
        match self {
            AuthProvider::Password => "PASSWORD".to_string(),
            AuthProvider::Google => "GOOGLE".to_string(),
            AuthProvider::Github => "GITHUB".to_string(),
        }
    }
}
