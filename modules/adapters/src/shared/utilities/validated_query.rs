use crate::shared::models::failure::HttpFailure;
use axum::extract::{FromRequestParts, Query};
use axum::http::request::Parts;
use serde::de::DeserializeOwned;
use shared::models::failure::Failure;
use validator::Validate;

pub struct ValidatedQuery<T>(pub T);

impl<T, S> FromRequestParts<S> for ValidatedQuery<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
{
    type Rejection = HttpFailure;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let query = Query::<T>::from_request_parts(parts, state)
            .await
            .map_err(|err| HttpFailure::new(Failure::BadRequest(err.to_string())))?;

        query.0.validate().map_err(|err| HttpFailure::new(Failure::ValidationError(err.to_string())))?;
        Ok(ValidatedQuery(query.0))
    }
}
