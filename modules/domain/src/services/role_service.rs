use async_trait::async_trait;
use std::sync::Arc;

// shared modules
use shared::types::DomainResponse;

// internal modules
use crate::entities::role_entity::RoleEntity;
use crate::repositories::role_repository::RoleRepository;
use crate::values::roles::Role;

#[async_trait]
pub trait RoleService: Send + Sync {
    async fn find_role_by_account_id(&self, account_id: &str) -> DomainResponse<Option<RoleEntity>>;
    async fn create_role(&self, account_id: &str, role_name: &str) -> DomainResponse<RoleEntity>;
    async fn find_and_update_role_by_account_id(&self, account_id: &str, role_name: &str)
    -> DomainResponse<RoleEntity>;
}

pub struct RoleServiceImpl {
    repository: Arc<dyn RoleRepository>,
}

impl RoleServiceImpl {
    pub fn new(repository: Arc<dyn RoleRepository>) -> Self {
        RoleServiceImpl { repository }
    }
}

#[async_trait]
impl RoleService for RoleServiceImpl {
    async fn find_role_by_account_id(&self, account_id: &str) -> DomainResponse<Option<RoleEntity>> {
        self.repository.find_by_account_id(account_id).await
    }

    async fn create_role(&self, account_id: &str, role_name: &str) -> DomainResponse<RoleEntity> {
        let role = Role::from_str(role_name)?;
        let role_entity = RoleEntity::new(false, account_id.to_string(), role)?;
        self.repository.create(&role_entity).await
    }

    async fn find_and_update_role_by_account_id(
        &self,
        account_id: &str,
        role_name: &str,
    ) -> DomainResponse<RoleEntity> {
        let role = Role::from_str(role_name)?;
        self.repository.find_and_update_role_by_account_id(account_id, &role.as_str().to_string()).await
    }
}
