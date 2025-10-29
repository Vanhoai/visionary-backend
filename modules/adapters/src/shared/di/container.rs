use once_cell::sync::Lazy;
use std::sync::Arc;

use crate::secondary::{
    apis::auth_api_impl::AuthApiImpl,
    repositories::{
        account_repository_impl::AccountRepositoryImpl, notification_repository_impl::NotificationRepositoryImpl,
        provider_repository_impl::ProviderRepositoryImpl, session_repository_impl::SessionRepositoryImpl,
    },
};

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

pub struct Container {
    // Repositories
    account_repository: Arc<dyn AccountRepository>,
    provider_repository: Arc<dyn ProviderRepository>,
    session_repository: Arc<dyn SessionRepository>,
    notification_repository: Arc<dyn NotificationRepository>,

    // Apis
    auth_api: Arc<dyn AuthApi>,

    // Domain Services
    auth_service: Arc<dyn AuthService>,
    account_service: Arc<dyn AccountService>,
    provider_service: Arc<dyn ProviderService>,
    notification_service: Arc<dyn NotificationService>,
    session_service: Arc<dyn SessionService>,

    // Application Services
    auth_app_service: Arc<AuthAppService>,
    notification_app_service: Arc<NotificationAppService>,
}

impl Container {
    pub fn new() -> Self {
        // Initialize repositories
        let account_repository = Self::create_account_repository();
        let provider_repository = Self::create_provider_repository();
        let session_repository = Self::create_session_repository();
        let notification_repository = Self::create_notification_repository();

        // Initialize apis
        let auth_api = Self::create_auth_api();

        // Initialize services
        let account_service = Self::create_account_service(account_repository.clone());
        let auth_service = Self::create_auth_service(account_repository.clone());
        let provider_service = Self::create_provider_service(provider_repository.clone());
        let notification_service = Self::create_notification_service(notification_repository.clone());
        let session_service = Self::create_session_service(session_repository.clone());

        // Initialize application services
        let auth_app_service = Self::create_auth_app_service(
            auth_service.clone(),
            account_service.clone(),
            provider_service.clone(),
            auth_api.clone(),
        );

        let notification_app_service = Self::create_notification_app_service(notification_service.clone());

        Container {
            // Repositories
            account_repository,
            provider_repository,
            session_repository,
            notification_repository,

            // Apis
            auth_api,

            // Domain Services
            auth_service,
            account_service,
            provider_service,
            notification_service,
            session_service,

            // Application Services
            auth_app_service,
            notification_app_service,
        }
    }

    // Repository factories
    fn create_account_repository() -> Arc<dyn AccountRepository> {
        Arc::new(AccountRepositoryImpl::new())
    }

    fn create_provider_repository() -> Arc<dyn ProviderRepository> {
        Arc::new(ProviderRepositoryImpl::new())
    }

    fn create_session_repository() -> Arc<dyn SessionRepository> {
        Arc::new(SessionRepositoryImpl::new())
    }

    fn create_notification_repository() -> Arc<dyn NotificationRepository> {
        Arc::new(NotificationRepositoryImpl::new())
    }

    // Apis factories
    fn create_auth_api() -> Arc<dyn AuthApi> {
        Arc::new(AuthApiImpl::new())
    }

    // Service factories
    fn create_account_service(repository: Arc<dyn AccountRepository>) -> Arc<dyn AccountService> {
        Arc::new(AccountServiceImpl::new(repository))
    }

    fn create_auth_service(repository: Arc<dyn AccountRepository>) -> Arc<dyn AuthService> {
        Arc::new(AuthServiceImpl::new(repository))
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
        auth_api: Arc<dyn AuthApi>,
    ) -> Arc<AuthAppService> {
        Arc::new(AuthAppService::new(auth_service, account_service, provider_service, auth_api))
    }

    fn create_notification_app_service(
        notification_service: Arc<dyn NotificationService>,
    ) -> Arc<NotificationAppService> {
        Arc::new(NotificationAppService::new(notification_service))
    }

    // Getters
    pub fn auth_app_service(&self) -> Arc<AuthAppService> {
        self.auth_app_service.clone()
    }
    
    pub fn notification_app_service(&self) -> Arc<NotificationAppService> {
        self.notification_app_service.clone()
    }
}

pub static DI_CONTAINER: Lazy<Arc<Container>> = Lazy::new(|| Arc::new(Container::new()));
