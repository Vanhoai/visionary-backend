use std::{sync::Arc, time::Duration};

use mongodb::{
    Client, Database,
    options::{ClientOptions, ServerApi, ServerApiVersion},
};
use shared::configs::APP_CONFIG;
use tokio::sync::OnceCell;

pub static ACCOUNT_TABLE: &str = "accounts";
pub static SESSION_TABLE: &str = "sessions";
pub static PROVIDER_TABLE: &str = "providers";
pub static WORK_TABLE: &str = "experiences";

pub static MONGO_CLIENT: OnceCell<Arc<Database>> = OnceCell::const_new();
pub async fn mongo_client() -> Arc<Database> {
    MONGO_CLIENT
        .get_or_init(|| async {
            let uri = APP_CONFIG.database.mongo_uri.clone();

            let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
            let mut client_options = ClientOptions::parse(uri).await.unwrap();
            client_options.server_api = Some(server_api);

            // Configure connection pool
            client_options.min_pool_size = Some(10);
            client_options.max_pool_size = Some(100);
            client_options.connect_timeout = Some(Duration::from_secs(5));
            client_options.server_selection_timeout = Some(Duration::from_secs(5));

            let client = Client::with_options(client_options).unwrap();
            Arc::new(client.database(&APP_CONFIG.database.mongo_database))
        })
        .await
        .clone()
}
