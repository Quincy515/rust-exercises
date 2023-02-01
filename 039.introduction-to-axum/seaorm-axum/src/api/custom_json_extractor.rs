use axum::{
    async_trait,
    body::HttpBody,
    extract::FromRequest,
    http::{Request, StatusCode},
    BoxError, Json, RequestExt,
};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Serialize, Validate)]
pub struct RequestUser {
    #[validate(email(message = "must be a valid email"))]
    pub username: String,
    #[validate(length(min = 6, message = "must be at least 6 characters"))]
    pub password: String,
}

#[async_trait]
impl<S, B> FromRequest<S, B> for RequestUser
where
    S: Send + Sync,
    B: HttpBody + Send + 'static,
    B::Data: Send,
    B::Error: Into<BoxError>,
{
    type Rejection = (StatusCode, String);
    async fn from_request(req: Request<B>, _state: &S) -> Result<Self, Self::Rejection> {
        let Json(user) = req
            .extract::<Json<RequestUser>, _>()
            .await
            .map_err(|err| (StatusCode::BAD_REQUEST, format!("{}", err)))?;

        if let Err(err) = user.validate() {
            return Err((StatusCode::BAD_REQUEST, err.to_string()));
        }
        Ok(user)
    }
}

pub async fn custom_json_extractor(user: RequestUser) -> Json<RequestUser> {
    Json(user)
}
