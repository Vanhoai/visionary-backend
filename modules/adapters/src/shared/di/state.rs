use shared::configs::APP_CONFIG;
use std::sync::Arc;
use tracing::info;

// shared modules
use domain::applications::{
    account_app_service::AccountAppService, auth_app_service::AuthAppService, category_app_service::CategoryAppService,
    global_app_service::GlobalAppService, notification_app_service::NotificationAppService,
    session_app_service::SessionAppService,
};

// internal modules
use crate::{
    secondary::apis::auth_api_impl::AuthApiImpl,
    shared::{
        di::{repository_module, service_module},
        utilities::databases::DatabaseType,
    },
};

#[derive(Clone)]
pub struct AppState {
    pub auth_app_service: Arc<AuthAppService>,
    pub account_app_service: Arc<AccountAppService>,
    pub notification_app_service: Arc<NotificationAppService>,
    pub session_app_service: Arc<SessionAppService>,
    pub category_app_service: Arc<CategoryAppService>,
    pub global_app_service: Arc<GlobalAppService>,
}

impl AppState {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        info!("📦 Initializing AppState");

        // Initialize repository module
        let database_type = DatabaseType::from_string(&APP_CONFIG.database.database_type)?;
        info!("📦 Using database type: {}", database_type.as_str());
        let repositories = repository_module::build_repository_module(database_type).await;
        info!("📦 Repositories initialized");

        // Initialize apis
        let auth_api = Arc::new(AuthApiImpl::new());

        // Initialize services
        let services = service_module::build_service_module(repositories.clone());
        info!("📦 Services initialized");

        // Initialize application services
        let auth_app_service = Arc::new(AuthAppService::new(
            services.get_auth_service(),
            services.get_account_service(),
            services.get_provider_service(),
            services.get_session_service(),
            services.get_role_service(),
            auth_api.clone(),
        ));

        let notification_app_service = Arc::new(NotificationAppService::new(services.get_notification_service()));
        let session_app_service = Arc::new(SessionAppService::new(services.get_session_service()));

        let account_app_service = Arc::new(AccountAppService::new(
            services.get_account_service(),
            services.get_role_service(),
            services.get_blog_service(),
        ));

        let category_app_service = Arc::new(CategoryAppService::new(services.get_category_service()));
        let global_app_service = Arc::new(GlobalAppService::new(services.get_experience_service()));

        info!("📦 AppState initialized successfully");
        Ok(AppState {
            auth_app_service,
            account_app_service,
            notification_app_service,
            session_app_service,
            category_app_service,
            global_app_service,
        })
    }
}
