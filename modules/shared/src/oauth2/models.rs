use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GoogleAccountInformation {
    pub id: String,
    pub email: String,
    pub verified_email: bool,
    pub name: String,
    pub given_name: String,
    pub family_name: String,
    pub picture: String,
}

#[derive(Debug, Deserialize)]
pub struct GitHubUserInfo {
    pub id: i64,
    pub email: Option<String>,
    pub name: Option<String>,
    pub login: String,
}
