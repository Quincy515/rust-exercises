use axum::http::{HeaderMap, HeaderValue};

pub async fn mirror_custom_header(headers: HeaderMap) -> String {
    let default_value = HeaderValue::from_static("no token");
    let value = headers.get("x-token").unwrap_or(&default_value);
    value.to_str().unwrap().to_owned()
}
