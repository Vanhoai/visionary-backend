use mongodb::Collection;
use std::sync::Arc;

use crate::secondary::repositories::mongodb::mongo_work_repository::MongoExperienceRepository;
use crate::secondary::repositories::mongodb::schemas::experience_schema::ExperienceSchema;
use crate::secondary::{
    apis::auth_api_impl::AuthApiImpl,
    repositories::mongodb::{
        mongo_account_repository::MongoAccountRepository,
        mongo_notification_repository::MongoNotificationRepository,
        mongo_provider_repository::MongoProviderRepository,
        mongo_session_repository::MongoSessionRepository,
        schemas::{account_schema::AccountSchema, provider_schema::ProviderSchema, session_schema::SessionSchema},
    },
};
use crate::shared::utilities::databases;
use domain::{
    apis::auth_api::AuthApi,
    applications::{
        account_app_service::AccountAppService, auth_app_service::AuthAppService,
        notification_app_service::NotificationAppService, session_app_service::SessionAppService,
    },
    repositories::{
        account_repository::AccountRepository, experience_repository::ExperienceRepository,
        notification_repository::NotificationRepository, provider_repository::ProviderRepository,
        session_repository::SessionRepository,
    },
    services::{
        account_service::{AccountService, AccountServiceImpl},
        auth_service::{AuthService, AuthServiceImpl},
        experience_service::{ExperienceService, ExperienceServiceImpl},
        notification_service::{NotificationService, NotificationServiceImpl},
        provider_service::{ProviderService, ProviderServiceImpl},
        session_service::{SessionService, SessionServiceImpl},
    },
};
use shared::models::filters::MongoFilter;

#[derive(Clone)]
pub struct AppState {
    pub auth_app_service: Arc<AuthAppService>,
    pub account_app_service: Arc<AccountAppService>,
    pub notification_app_service: Arc<NotificationAppService>,
    pub session_app_service: Arc<SessionAppService>,
}

impl AppState {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        // Initialize database connection
        let mongodb_connection = databases::mongo_client().await;

        // Initialize collections
        let account_collection = Arc::new(mongodb_connection.collection(databases::ACCOUNT_TABLE));
        let provider_collection = Arc::new(mongodb_connection.collection(databases::PROVIDER_TABLE));
        let session_collection = Arc::new(mongodb_connection.collection(databases::SESSION_TABLE));
        let work_collection = Arc::new(mongodb_connection.collection(databases::WORK_TABLE));

        // Initialize repositories
        let account_repository = Self::create_account_repository(account_collection.clone());
        let provider_repository = Self::create_provider_repository(provider_collection.clone());
        let session_repository = Self::create_session_repository(session_collection.clone());
        let notification_repository = Self::create_notification_repository();
        let work_repository = Self::create_work_repository(work_collection.clone());

        // Initialize apis
        let auth_api = Self::create_auth_api();

        // Initialize services
        let account_service = Self::create_account_service(account_repository.clone());
        let auth_service = Self::create_auth_service();
        let provider_service = Self::create_provider_service(provider_repository.clone());
        let notification_service = Self::create_notification_service(notification_repository.clone());
        let session_service = Self::create_session_service(session_repository.clone());
        let experience_service = Self::create_experience_service(work_repository.clone());

        // Initialize application services
        let auth_app_service = Self::create_auth_app_service(
            auth_service.clone(),
            account_service.clone(),
            provider_service.clone(),
            session_service.clone(),
            auth_api.clone(),
        );

        let notification_app_service = Self::create_notification_app_service(notification_service.clone());
        let session_app_service = Self::create_session_app_service(session_service.clone());
        let account_app_service = Self::create_account_app_service(account_service.clone(), experience_service.clone());

        // Return AppState
        Ok(AppState { auth_app_service, account_app_service, notification_app_service, session_app_service })
    }

    // Repository factories
    fn create_account_repository(collection: Arc<Collection<AccountSchema>>) -> Arc<dyn AccountRepository> {
        Arc::new(MongoAccountRepository::new(collection))
    }

    fn create_provider_repository(collection: Arc<Collection<ProviderSchema>>) -> Arc<dyn ProviderRepository> {
        Arc::new(MongoProviderRepository::new(collection))
    }

    fn create_session_repository(collection: Arc<Collection<SessionSchema>>) -> Arc<dyn SessionRepository> {
        Arc::new(MongoSessionRepository::new(collection))
    }

    fn create_notification_repository() -> Arc<dyn NotificationRepository> {
        Arc::new(MongoNotificationRepository::new())
    }

    fn create_work_repository(collection: Arc<Collection<ExperienceSchema>>) -> Arc<dyn ExperienceRepository> {
        Arc::new(MongoExperienceRepository::new(collection))
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

    fn create_experience_service(repository: Arc<dyn ExperienceRepository>) -> Arc<dyn ExperienceService> {
        Arc::new(ExperienceServiceImpl::new(repository))
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

    fn create_session_app_service(session_service: Arc<dyn SessionService>) -> Arc<SessionAppService> {
        Arc::new(SessionAppService::new(session_service))
    }

    fn create_account_app_service(
        account_service: Arc<dyn AccountService>,
        experience_service: Arc<dyn ExperienceService>,
    ) -> Arc<AccountAppService> {
        Arc::new(AccountAppService::new(account_service, experience_service))
    }
}
