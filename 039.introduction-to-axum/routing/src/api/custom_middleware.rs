use axum::Extension;

#[derive(Clone)]
pub struct HeaderMessage(pub String);

pub async fn custom_middleware(Extension(message): Extension<HeaderMessage>) -> String {
    message.0
}
