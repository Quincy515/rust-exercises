use axum::{
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};

use super::custom_middleware::HeaderMessage;

pub async fn set_custom_middleware<B>(
    mut request: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    // do something with `request`...
    let headers = request.headers();
    let message = headers
        .get("message")
        .ok_or_else(|| StatusCode::BAD_REQUEST)?;
    let message = message
        .to_str()
        .map_err(|_| StatusCode::BAD_REQUEST)?
        .to_owned();
    request.extensions_mut().insert(HeaderMessage(message));

    let response = next.run(request).await;

    // do something with `response`...

    Ok(response)
}
