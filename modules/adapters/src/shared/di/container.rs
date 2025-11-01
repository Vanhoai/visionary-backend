use mongodb::bson::doc;
use mongodb::options::{ClientOptions, ServerApi, ServerApiVersion};
use mongodb::{Client, Collection, Database};
use once_cell::sync::Lazy;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::OnceCell;

use crate::secondary::apis::auth_api_impl::AuthApiImpl;
use crate::secondary::repositories::mongodb::mongo_account_repository::MongoAccountRepository;
use crate::secondary::repositories::mongodb::mongo_notification_repository::MongoNotificationRepository;
use crate::secondary::repositories::mongodb::mongo_provider_repository::MongoProviderRepository;
use crate::secondary::repositories::mongodb::mongo_session_repository::MongoSessionRepository;
use domain::entities::account_entity::AccountEntity;
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
use shared::configs::APP_CONFIG;

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
    pub async fn new() -> Self {
        // Initialize database connection
        let mongodb_connection = Self::create_mongo_connection().await;

        // Initialize collections
        let account_collection = Arc::new(mongodb_connection.collection("accounts"));

        // Initialize repositories
        let account_repository = Self::create_account_repository(account_collection.clone());
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

    // Database
    async fn create_mongo_connection() -> Database {
        let uri = APP_CONFIG.database.mongo_uri.clone();

        let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
        let mut client_options = ClientOptions::parse(uri).await.unwrap();
        client_options.server_api = Some(server_api);

        // Configure connection pool
        client_options.min_pool_size = Some(10);
        client_options.max_pool_size = Some(100);
        client_options.connect_timeout = Some(Duration::from_secs(5));
        client_options.server_selection_timeout = Some(Duration::from_secs(5));

        // Create a new client and connect to the server
        let client = Client::with_options(client_options).unwrap();
        client.database(&APP_CONFIG.database.mongo_database)
    }

    // Repository factories
    fn create_account_repository(collection: Arc<Collection<AccountEntity>>) -> Arc<dyn AccountRepository> {
        Arc::new(MongoAccountRepository::new(collection))
    }

    fn create_provider_repository() -> Arc<dyn ProviderRepository> {
        Arc::new(MongoProviderRepository::new())
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

static DI_CONTAINER: OnceCell<Arc<Container>> = OnceCell::const_new();
pub async fn instance() -> Arc<Container> {
    DI_CONTAINER.get_or_init(|| async { Arc::new(Container::new().await) }).await.clone()
}
