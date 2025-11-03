use async_trait::async_trait;
use domain::apis::auth_api::AuthApi;

pub struct AuthApiImpl {}

impl AuthApiImpl {
    pub fn new() -> Self {
        AuthApiImpl {}
    }
}

#[async_trait]
impl AuthApi for AuthApiImpl {}
