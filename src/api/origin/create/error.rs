use crate::{services::origin::error::ServiceError as OriginServiceError, Details};
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

#[derive(thiserror::Error, Debug)]
pub enum AddOriginError {
    #[error(transparent)]
    OriginServiceError(#[from] OriginServiceError),
}

impl From<AddOriginError> for StatusCode {
    fn from(value: AddOriginError) -> Self {
        match value {
            AddOriginError::OriginServiceError(error) => Self::from(error),
        }
    }
}

impl IntoResponse for AddOriginError {
    fn into_response(self) -> Response {
        let reason = self.to_string();
        let status_code: StatusCode = self.into();

        tracing::error!(%reason, %status_code, "Error occurred while trying to handle request!");
        (status_code, Json(Details { reason })).into_response()
    }
}
