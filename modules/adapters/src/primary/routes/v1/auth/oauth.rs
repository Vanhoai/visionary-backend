use crate::shared::models::failure::HttpFailure;
use crate::shared::models::response::HttpResponse;
use shared::models::failure::Failure;

pub async fn execute() -> Result<HttpResponse<()>, HttpFailure> {
    let failure = Failure::UnknownFailure("Invalid credentials".to_string());
    Err(HttpFailure::new(failure))
}
