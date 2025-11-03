use mongodb::Collection;
use std::sync::Arc;

use crate::secondary::{
    apis::auth_api_impl::AuthApiImpl,
    repositories::mongodb::{
        mongo_account_repository::MongoAccountRepository,
        mongo_notification_repository::MongoNotificationRepository,
        mongo_provider_repository::MongoProviderRepository,
        mongo_session_repository::MongoSessionRepository,
        schemas::{account_schema::AccountSchema, provider_schema::ProviderSchema},
    },
};
use crate::shared::utilities::databases;
use domain::{
    apis::auth_api::AuthApi,
    applications::{auth_app_service::AuthAppService, notification_app_service::NotificationAppService},
    repositories::{
        account_repository::AccountRepository, notification_repository::NotificationRepository,
        provider_repository::ProviderRepository, session_repository::SessionRepository,
    },
    services::{
        account_service::{AccountService, AccountServiceImpl},
        auth_service::{AuthService, AuthServiceImpl},
        notification_service::{NotificationService, NotificationServiceImpl},
        provider_service::{ProviderService, ProviderServiceImpl},
        session_service::{SessionService, SessionServiceImpl},
    },
};

#[derive(Clone)]
pub struct AppState {
    pub auth_app_service: Arc<AuthAppService>,
    pub notification_app_service: Arc<NotificationAppService>,
}

impl AppState {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        // Initialize database connection
        let mongodb_connection = databases::mongo_client().await;

        // Initialize collections
        let account_collection = Arc::new(mongodb_connection.collection(databases::ACCOUNT_TABLE));
        let provider_collection = Arc::new(mongodb_connection.collection(databases::PROVIDER_TABLE));

        // Initialize repositories
        let account_repository = Self::create_account_repository(account_collection.clone());
        let provider_repository = Self::create_provider_repository(provider_collection.clone());
        let session_repository = Self::create_session_repository();
        let notification_repository = Self::create_notification_repository();

        // Initialize apis
        let auth_api = Self::create_auth_api();

        // Initialize services
        let account_service = Self::create_account_service(account_repository.clone());
        let auth_service = Self::create_auth_service();
        let provider_service = Self::create_provider_service(provider_repository.clone());
        let notification_service = Self::create_notification_service(notification_repository.clone());
        let session_service = Self::create_session_service(session_repository.clone());

        // Initialize application services
        let auth_app_service = Self::create_auth_app_service(
            auth_service.clone(),
            account_service.clone(),
            provider_service.clone(),
            session_service.clone(),
            auth_api.clone(),
        );

        let notification_app_service = Self::create_notification_app_service(notification_service.clone());

        // Return AppState
        Ok(AppState { auth_app_service, notification_app_service })
    }

    // Repository factories
    fn create_account_repository(collection: Arc<Collection<AccountSchema>>) -> Arc<dyn AccountRepository> {
        Arc::new(MongoAccountRepository::new(collection))
    }

    fn create_provider_repository(collection: Arc<Collection<ProviderSchema>>) -> Arc<dyn ProviderRepository> {
        Arc::new(MongoProviderRepository::new(collection))
    }

    fn create_session_repository() -> Arc<dyn SessionRepository> {
        Arc::new(MongoSessionRepository::new())
    }

    fn create_notification_repository() -> Arc<dyn NotificationRepository> {
        Arc::new(MongoNotificationRepository::new())
    }

    // Apis factories
    fn create_auth_api() -> Arc<dyn AuthApi> {
        Arc::new(AuthApiImpl::new())
    }

    // Service factories
    fn create_account_service(repository: Arc<dyn AccountRepository>) -> Arc<dyn AccountService> {
        Arc::new(AccountServiceImpl::new(repository))
    }

    fn create_auth_service() -> Arc<dyn AuthService> {
        Arc::new(AuthServiceImpl::new())
    }

    fn create_provider_service(repository: Arc<dyn ProviderRepository>) -> Arc<dyn ProviderService> {
        Arc::new(ProviderServiceImpl::new(repository))
    }

    fn create_notification_service(repository: Arc<dyn NotificationRepository>) -> Arc<dyn NotificationService> {
        Arc::new(NotificationServiceImpl::new(repository))
    }

    fn create_session_service(repository: Arc<dyn SessionRepository>) -> Arc<dyn SessionService> {
        Arc::new(SessionServiceImpl::new(repository))
    }

    // Application service factories
    fn create_auth_app_service(
        auth_service: Arc<dyn AuthService>,
        account_service: Arc<dyn AccountService>,
        provider_service: Arc<dyn ProviderService>,
        session_service: Arc<dyn SessionService>,
        auth_api: Arc<dyn AuthApi>,
    ) -> Arc<AuthAppService> {
        Arc::new(AuthAppService::new(auth_service, account_service, provider_service, session_service, auth_api))
    }

    fn create_notification_app_service(
        notification_service: Arc<dyn NotificationService>,
    ) -> Arc<NotificationAppService> {
        Arc::new(NotificationAppService::new(notification_service))
    }
}
