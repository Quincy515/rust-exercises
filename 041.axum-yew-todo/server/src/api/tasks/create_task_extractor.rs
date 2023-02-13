use axum::{
    async_trait,
    body::HttpBody,
    extract::FromRequest,
    http::{Request, StatusCode},
    BoxError, Json, RequestExt,
};
use serde::Deserialize;
use validator::Validate;

use crate::util::app_error::AppError;

#[derive(Deserialize, Validate)]
pub struct ValidateCreateTask {
    #[validate(length(min = 1, max = 1))]
    pub priority: Option<String>,
    #[validate(
        required(message = "missing task title"),
        length(min = 1, max = 6, message = "task title length should >1 and <7")
    )]
    pub title: Option<String>,
    pub description: Option<String>,
}

#[async_trait]
impl<S, B> FromRequest<S, B> for ValidateCreateTask
where
    B: HttpBody + Send + 'static,
    B::Data: Send,
    B::Error: Into<BoxError>,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request(req: Request<B>, _state: &S) -> Result<Self, Self::Rejection> {
        let Json(task) = req
            .extract::<Json<ValidateCreateTask>, _>()
            .await
            .map_err(|err| {
                eprintln!("Error getting bytes in custom create task extractor: {err:?}");
                AppError::new(StatusCode::BAD_REQUEST, "Internal server error")
            })?;

        if let Err(errors) = task.validate() {
            let field_errors = errors.field_errors();
            for (_, error) in field_errors {
                return Err(AppError::new(
                    StatusCode::BAD_REQUEST,
                    error.first().unwrap().clone().message.unwrap().to_string(),
                ));
            }
        }

        Ok(task)
    }
}
