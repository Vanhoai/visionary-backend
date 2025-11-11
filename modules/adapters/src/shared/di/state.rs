use mongodb::Collection;
use std::sync::Arc;

// shared modules
use domain::{
    apis::auth_api::AuthApi,
    applications::{
        account_app_service::AccountAppService, auth_app_service::AuthAppService,
        category_app_service::CategoryAppService, notification_app_service::NotificationAppService,
        session_app_service::SessionAppService,
    },
    repositories::{
        account_repository::AccountRepository, category_repository::CategoryRepository,
        experience_repository::ExperienceRepository, notification_repository::NotificationRepository,
        provider_repository::ProviderRepository, role_repository::RoleRepository,
        session_repository::SessionRepository,
    },
    services::{
        account_service::{AccountService, AccountServiceImpl},
        auth_service::{AuthService, AuthServiceImpl},
        category_service::{CategoryService, CategoryServiceImpl},
        experience_service::{ExperienceService, ExperienceServiceImpl},
        notification_service::{NotificationService, NotificationServiceImpl},
        provider_service::{ProviderService, ProviderServiceImpl},
        role_service::{RoleService, RoleServiceImpl},
        session_service::{SessionService, SessionServiceImpl},
    },
};

// internal modules
use crate::secondary::{
    apis::auth_api_impl::AuthApiImpl,
    repositories::mongodb::{
        mongo_account_repository::MongoAccountRepository,
        mongo_category_repository::MongoCategoryRepository,
        mongo_experience_repository::MongoExperienceRepository,
        mongo_notification_repository::MongoNotificationRepository,
        mongo_provider_repository::MongoProviderRepository,
        mongo_role_repository::MongoRoleRepository,
        mongo_session_repository::MongoSessionRepository,
        schemas::{
            account_schema::AccountSchema, category_schema::CategorySchema, experience_schema::ExperienceSchema,
            provider_schema::ProviderSchema, role_schema::RoleSchema, session_schema::SessionSchema,
        },
    },
};
use crate::shared::utilities::databases;

#[derive(Clone)]
pub struct AppState {
    pub auth_app_service: Arc<AuthAppService>,
    pub account_app_service: Arc<AccountAppService>,
    pub notification_app_service: Arc<NotificationAppService>,
    pub session_app_service: Arc<SessionAppService>,
    pub category_app_service: Arc<CategoryAppService>,
}

impl AppState {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        // Initialize database connection
        let mongodb_connection = databases::mongo_client().await;

        // Initialize collections
        let account_collection = Arc::new(mongodb_connection.collection(databases::ACCOUNT_TABLE));
        let provider_collection = Arc::new(mongodb_connection.collection(databases::PROVIDER_TABLE));
        let session_collection = Arc::new(mongodb_connection.collection(databases::SESSION_TABLE));
        let experience_collection = Arc::new(mongodb_connection.collection(databases::EXPERIENCE_TABLE));
        let role_collection = Arc::new(mongodb_connection.collection(databases::ROLE_TABLE));
        let category_collection = Arc::new(mongodb_connection.collection(databases::CATEGORY_TABLE));

        // Initialize repositories
        let account_repository = Self::create_account_repository(account_collection.clone());
        let provider_repository = Self::create_provider_repository(provider_collection.clone());
        let session_repository = Self::create_session_repository(session_collection.clone());
        let notification_repository = Self::create_notification_repository();
        let work_repository = Self::create_work_repository(experience_collection.clone());
        let role_repository = Self::create_role_repository(role_collection.clone());
        let category_repository = Self::create_category_repository(category_collection.clone());

        // Initialize apis
        let auth_api = Self::create_auth_api();

        // Initialize services
        let account_service = Self::create_account_service(account_repository.clone());
        let auth_service = Self::create_auth_service();
        let provider_service = Self::create_provider_service(provider_repository.clone());
        let notification_service = Self::create_notification_service(notification_repository.clone());
        let session_service = Self::create_session_service(session_repository.clone());
        let experience_service = Self::create_experience_service(work_repository.clone());
        let role_service = Self::create_role_service(role_repository.clone());
        let category_service = Self::create_category_service(category_repository.clone());

        // Initialize application services
        let auth_app_service = Self::create_auth_app_service(
            auth_service.clone(),
            account_service.clone(),
            provider_service.clone(),
            session_service.clone(),
            role_service.clone(),
            auth_api.clone(),
        );

        let notification_app_service = Self::create_notification_app_service(notification_service.clone());
        let session_app_service = Self::create_session_app_service(session_service.clone());
        let account_app_service =
            Self::create_account_app_service(account_service.clone(), experience_service.clone(), role_service.clone());
        let category_app_service = Self::create_category_app_service(category_service.clone());

        // Return AppState
        Ok(AppState {
            auth_app_service,
            account_app_service,
            notification_app_service,
            session_app_service,
            category_app_service,
        })
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

    fn create_role_repository(collection: Arc<Collection<RoleSchema>>) -> Arc<dyn RoleRepository> {
        Arc::new(MongoRoleRepository::new(collection))
    }

    fn create_category_repository(collection: Arc<Collection<CategorySchema>>) -> Arc<dyn CategoryRepository> {
        Arc::new(MongoCategoryRepository::new(collection))
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

    fn create_role_service(repository: Arc<dyn RoleRepository>) -> Arc<dyn RoleService> {
        Arc::new(RoleServiceImpl::new(repository))
    }

    fn create_category_service(repository: Arc<dyn CategoryRepository>) -> Arc<dyn CategoryService> {
        Arc::new(CategoryServiceImpl::new(repository))
    }

    // Application service factories
    fn create_auth_app_service(
        auth_service: Arc<dyn AuthService>,
        account_service: Arc<dyn AccountService>,
        provider_service: Arc<dyn ProviderService>,
        session_service: Arc<dyn SessionService>,
        role_service: Arc<dyn RoleService>,
        auth_api: Arc<dyn AuthApi>,
    ) -> Arc<AuthAppService> {
        Arc::new(AuthAppService::new(
            auth_service,
            account_service,
            provider_service,
            session_service,
            role_service,
            auth_api,
        ))
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
        role_service: Arc<dyn RoleService>,
    ) -> Arc<AccountAppService> {
        Arc::new(AccountAppService::new(account_service, experience_service, role_service))
    }

    fn create_category_app_service(category_service: Arc<dyn CategoryService>) -> Arc<CategoryAppService> {
        Arc::new(CategoryAppService::new(category_service))
    }
}
