use axum::extract::Path;

pub async fn path_variable(Path(id): Path<u32>) -> String {
    id.to_string()
}

pub async fn hard_coded_path() -> String {
    "You get 15!".to_owned()
}
