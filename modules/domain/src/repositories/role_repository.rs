use crate::entities::role_entity::RoleEntity;
use crate::repositories::base_repository::BaseRepository;
use async_trait::async_trait;
use shared::types::DomainResponse;

#[async_trait]
pub trait RoleRepository: BaseRepository<RoleEntity> {
    async fn find_by_account_id(&self, account_id: &str) -> DomainResponse<Option<RoleEntity>>;
    async fn find_and_update_role_by_account_id(&self, account_id: &str, role_name: &str)
    -> DomainResponse<RoleEntity>;
}
