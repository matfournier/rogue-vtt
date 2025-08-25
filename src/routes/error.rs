use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use validator::ValidationErrors;

// #[derive(Debug)]
pub enum HttpError {
    Sqlx(sqlx::Error),
    Anyhow(anyhow::Error),
    Invalid(ValidationErrors),
    Unauthenticated,
}

impl From<ValidationErrors> for HttpError {
    fn from(err: ValidationErrors) -> Self {
        Self::Invalid(err)
    }
}
impl From<sqlx::Error> for HttpError {
    fn from(err: sqlx::Error) -> Self {
        Self::Sqlx(err)
    }
}
impl From<anyhow::Error> for HttpError {
    fn from(err: anyhow::Error) -> Self {
        Self::Anyhow(err)
    }
}

impl HttpError {
    fn status_code(&self) -> StatusCode {
        use HttpError::*;

        match self {
            Sqlx(_) | Anyhow(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Invalid(_) | Unauthenticated => StatusCode::UNPROCESSABLE_ENTITY,
        }
    }
}

impl IntoResponse for HttpError {
    fn into_response(self) -> Response {
        use HttpError::*;
        #[derive(Serialize)]
        struct ErrorResponse {
            message: String,
        }

        // todo dig out Sqlx/Anyhow errors for logging potentially
        // to get rid of the field `0` is never read lint check

        let message = match &self {
            Sqlx(_) | Anyhow(_) => "Something went wrong".to_string(),
            Unauthenticated => "invalid username and/or password".into(),
            Invalid(ref e) => match serde_json::to_string(e.errors()) {
                Ok(v) => v,
                Err(_) => "Unexpected error".to_string(),
            },
        };

        (self.status_code(), Json(ErrorResponse { message })).into_response()
    }
}
