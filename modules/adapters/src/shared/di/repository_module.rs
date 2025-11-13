use std::sync::Arc;

// shared modules
use domain::repositories::{
    account_repository::AccountRepository, blog_repository::BlogRepository, category_repository::CategoryRepository,
    experience_repository::ExperienceRepository, notification_repository::NotificationRepository,
    provider_repository::ProviderRepository, role_repository::RoleRepository, session_repository::SessionRepository,
};
use shared::configs::APP_CONFIG;

// internal modules
use crate::secondary::repositories::{
    mongodb::{
        mongo_account_repository::MongoAccountRepository, mongo_blog_repository::MongoBlogRepository,
        mongo_category_repository::MongoCategoryRepository, mongo_experience_repository::MongoExperienceRepository,
        mongo_notification_repository::MongoNotificationRepository, mongo_provider_repository::MongoProviderRepository,
        mongo_role_repository::MongoRoleRepository, mongo_session_repository::MongoSessionRepository,
    },
    scylla::{
        scylla_account_repository::ScyllaAccountRepository, scylla_blog_repository::ScyllaBlogRepository,
        scylla_category_repository::ScyllaCategoryRepository, scylla_experience_repository::ScyllaExperienceRepository,
        scylla_notification_repository::ScyllaNotificationRepository,
        scylla_provider_repository::ScyllaProviderRepository, scylla_role_repository::ScyllaRoleRepository,
        scylla_session_repository::ScyllaSessionRepository,
    },
};
use crate::shared::utilities::databases::{
    ACCOUNT_TABLE, BLOG_TABLE, CATEGORY_TABLE, DatabaseType, EXPERIENCE_TABLE, NOTIFICATION_TABLE, PROVIDER_TABLE,
    ROLE_TABLE, SESSION_TABLE, mongo_client, scylla_session,
};

pub trait RepositoryModule: Send + Sync {
    fn get_account_repository(&self) -> Arc<dyn AccountRepository>;
    fn get_provider_repository(&self) -> Arc<dyn ProviderRepository>;
    fn get_session_repository(&self) -> Arc<dyn SessionRepository>;
    fn get_experience_repository(&self) -> Arc<dyn ExperienceRepository>;
    fn get_role_repository(&self) -> Arc<dyn RoleRepository>;
    fn get_category_repository(&self) -> Arc<dyn CategoryRepository>;
    fn get_notification_repository(&self) -> Arc<dyn NotificationRepository>;
    fn get_blog_repository(&self) -> Arc<dyn BlogRepository>;
}

#[macro_export]
macro_rules! impl_repository_module {
    (
        $repository_module:ident,
        $(
            $method_name:ident -> $trait_name:ident : $field_name:ident
        ),* $(,)?
    ) => {
        impl RepositoryModule for $repository_module {
            $(
                fn $method_name(&self) -> Arc<dyn $trait_name> {
                    Arc::clone(&self.$field_name)
                }
            )*
        }
    };
}

// MongoDB Implementation
struct MongoRepositoryModule {
    account_repository: Arc<dyn AccountRepository>,
    provider_repository: Arc<dyn ProviderRepository>,
    session_repository: Arc<dyn SessionRepository>,
    experience_repository: Arc<dyn ExperienceRepository>,
    role_repository: Arc<dyn RoleRepository>,
    category_repository: Arc<dyn CategoryRepository>,
    notification_repository: Arc<dyn NotificationRepository>,
    blog_repository: Arc<dyn BlogRepository>,
}

impl MongoRepositoryModule {
    async fn new() -> Self {
        let db = mongo_client().await;

        Self {
            account_repository: Arc::new(MongoAccountRepository::new(Arc::new(db.collection(ACCOUNT_TABLE)))),
            provider_repository: Arc::new(MongoProviderRepository::new(Arc::new(db.collection(PROVIDER_TABLE)))),
            session_repository: Arc::new(MongoSessionRepository::new(Arc::new(db.collection(SESSION_TABLE)))),
            experience_repository: Arc::new(MongoExperienceRepository::new(Arc::new(db.collection(EXPERIENCE_TABLE)))),
            role_repository: Arc::new(MongoRoleRepository::new(Arc::new(db.collection(ROLE_TABLE)))),
            category_repository: Arc::new(MongoCategoryRepository::new(Arc::new(db.collection(CATEGORY_TABLE)))),
            notification_repository: Arc::new(MongoNotificationRepository::new(Arc::new(
                db.collection(NOTIFICATION_TABLE),
            ))),
            blog_repository: Arc::new(MongoBlogRepository::new(Arc::new(db.collection(BLOG_TABLE)))),
        }
    }
}

impl_repository_module!(
    MongoRepositoryModule,
    get_account_repository -> AccountRepository: account_repository,
    get_provider_repository -> ProviderRepository: provider_repository,
    get_session_repository -> SessionRepository: session_repository,
    get_experience_repository -> ExperienceRepository: experience_repository,
    get_role_repository -> RoleRepository: role_repository,
    get_category_repository -> CategoryRepository: category_repository,
    get_notification_repository -> NotificationRepository: notification_repository,
    get_blog_repository -> BlogRepository: blog_repository,
);

// Scylla Implementation
struct ScyllaRepositoryModule {
    account_repository: Arc<dyn AccountRepository>,
    provider_repository: Arc<dyn ProviderRepository>,
    session_repository: Arc<dyn SessionRepository>,
    experience_repository: Arc<dyn ExperienceRepository>,
    role_repository: Arc<dyn RoleRepository>,
    category_repository: Arc<dyn CategoryRepository>,
    notification_repository: Arc<dyn NotificationRepository>,
    blog_repository: Arc<dyn BlogRepository>,
}

impl ScyllaRepositoryModule {
    async fn new() -> Self {
        let session = scylla_session().await;
        let keyspace = APP_CONFIG.database.scylla_keyspace.clone();

        Self {
            account_repository: Arc::new(ScyllaAccountRepository::new(session.clone(), &keyspace, ACCOUNT_TABLE)),
            provider_repository: Arc::new(ScyllaProviderRepository::new(session.clone(), &keyspace, PROVIDER_TABLE)),
            session_repository: Arc::new(ScyllaSessionRepository::new(session.clone(), &keyspace, SESSION_TABLE)),
            experience_repository: Arc::new(ScyllaExperienceRepository::new(
                session.clone(),
                &keyspace,
                EXPERIENCE_TABLE,
            )),
            role_repository: Arc::new(ScyllaRoleRepository::new(session.clone(), &keyspace, ROLE_TABLE)),
            category_repository: Arc::new(ScyllaCategoryRepository::new(session.clone(), &keyspace, CATEGORY_TABLE)),
            notification_repository: Arc::new(ScyllaNotificationRepository::new(
                session.clone(),
                &keyspace,
                NOTIFICATION_TABLE,
            )),
            blog_repository: Arc::new(ScyllaBlogRepository::new(session.clone(), &keyspace, BLOG_TABLE)),
        }
    }
}

impl_repository_module!(
    ScyllaRepositoryModule,
    get_account_repository -> AccountRepository: account_repository,
    get_provider_repository -> ProviderRepository: provider_repository,
    get_session_repository -> SessionRepository: session_repository,
    get_experience_repository -> ExperienceRepository: experience_repository,
    get_role_repository -> RoleRepository: role_repository,
    get_category_repository -> CategoryRepository: category_repository,
    get_notification_repository -> NotificationRepository: notification_repository,
    get_blog_repository -> BlogRepository: blog_repository,
);

// Factory function
pub async fn build_repository_module(database_type: DatabaseType) -> Arc<dyn RepositoryModule> {
    match database_type {
        DatabaseType::Mongo => Arc::new(MongoRepositoryModule::new().await),
        DatabaseType::Scylla => Arc::new(ScyllaRepositoryModule::new().await),
    }
}
