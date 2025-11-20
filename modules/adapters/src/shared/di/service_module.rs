use std::sync::Arc;

// shared modules
use domain::services::{
    account_service::{AccountService, AccountServiceImpl},
    auth_service::{AuthService, AuthServiceImpl},
    blog_service::{BlogService, BlogServiceImpl},
    category_service::{CategoryService, CategoryServiceImpl},
    experience_service::{ExperienceService, ExperienceServiceImpl},
    notification_service::{NotificationService, NotificationServiceImpl},
    project_service::{ProjectService, ProjectServiceImpl},
    provider_service::{ProviderService, ProviderServiceImpl},
    role_service::{RoleService, RoleServiceImpl},
    session_service::{SessionService, SessionServiceImpl},
};

// internal modules
use crate::shared::di::repository_module::RepositoryModule;

pub trait ServiceModule {
    fn get_auth_service(&self) -> Arc<dyn AuthService>;
    fn get_account_service(&self) -> Arc<dyn AccountService>;
    fn get_provider_service(&self) -> Arc<dyn ProviderService>;
    fn get_session_service(&self) -> Arc<dyn SessionService>;
    fn get_experience_service(&self) -> Arc<dyn ExperienceService>;
    fn get_role_service(&self) -> Arc<dyn RoleService>;
    fn get_category_service(&self) -> Arc<dyn CategoryService>;
    fn get_notification_service(&self) -> Arc<dyn NotificationService>;
    fn get_blog_service(&self) -> Arc<dyn BlogService>;
    fn get_project_service(&self) -> Arc<dyn ProjectService>;
}

pub fn build_service_module(repository_module: Arc<dyn RepositoryModule>) -> Arc<dyn ServiceModule> {
    Arc::new(ServiceModuleImpl::new(repository_module))
}

struct ServiceModuleImpl {
    auth_service: Arc<dyn AuthService>,
    account_service: Arc<dyn AccountService>,
    provider_service: Arc<dyn ProviderService>,
    session_service: Arc<dyn SessionService>,
    experience_service: Arc<dyn ExperienceService>,
    role_service: Arc<dyn RoleService>,
    category_service: Arc<dyn CategoryService>,
    notification_service: Arc<dyn NotificationService>,
    blog_service: Arc<dyn BlogService>,
    project_service: Arc<dyn ProjectService>,
}

impl ServiceModuleImpl {
    pub fn new(repository_module: Arc<dyn RepositoryModule>) -> Self {
        let auth_service = Arc::new(AuthServiceImpl::new());
        let account_service = Arc::new(AccountServiceImpl::new(repository_module.get_account_repository()));
        let provider_service = Arc::new(ProviderServiceImpl::new(repository_module.get_provider_repository()));
        let session_service = Arc::new(SessionServiceImpl::new(repository_module.get_session_repository()));
        let experience_service = Arc::new(ExperienceServiceImpl::new(repository_module.get_experience_repository()));
        let role_service = Arc::new(RoleServiceImpl::new(repository_module.get_role_repository()));
        let category_service = Arc::new(CategoryServiceImpl::new(repository_module.get_category_repository()));
        let notification_service =
            Arc::new(NotificationServiceImpl::new(repository_module.get_notification_repository()));
        let blog_service = Arc::new(BlogServiceImpl::new(repository_module.get_blog_repository()));
        let project_service = Arc::new(ProjectServiceImpl::new(repository_module.get_project_repository()));

        ServiceModuleImpl {
            auth_service,
            account_service,
            provider_service,
            session_service,
            experience_service,
            role_service,
            category_service,
            notification_service,
            blog_service,
            project_service,
        }
    }
}

#[macro_export]
macro_rules! impl_service_module {
    ($service_module:ident) => {
        impl ServiceModule for $service_module {
            fn get_auth_service(&self) -> Arc<dyn AuthService> {
                self.auth_service.clone()
            }

            fn get_account_service(&self) -> Arc<dyn AccountService> {
                self.account_service.clone()
            }

            fn get_provider_service(&self) -> Arc<dyn ProviderService> {
                self.provider_service.clone()
            }

            fn get_session_service(&self) -> Arc<dyn SessionService> {
                self.session_service.clone()
            }

            fn get_experience_service(&self) -> Arc<dyn ExperienceService> {
                self.experience_service.clone()
            }

            fn get_role_service(&self) -> Arc<dyn RoleService> {
                self.role_service.clone()
            }

            fn get_category_service(&self) -> Arc<dyn CategoryService> {
                self.category_service.clone()
            }

            fn get_notification_service(&self) -> Arc<dyn NotificationService> {
                self.notification_service.clone()
            }

            fn get_blog_service(&self) -> Arc<dyn BlogService> {
                self.blog_service.clone()
            }

            fn get_project_service(&self) -> Arc<dyn ProjectService> {
                self.project_service.clone()
            }
        }
    };
}

impl_service_module!(ServiceModuleImpl);
