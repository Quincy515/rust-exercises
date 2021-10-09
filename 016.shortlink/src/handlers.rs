use axum::extract::Extension;
use axum::http::StatusCode;
use axum::Json;
use axum::response::{Html, IntoResponse};
use sqlx::{MySql, Pool};
use crate::{dto, models};

pub async fn hello() -> Html<&'static str> {
    println!("hello");
    Html("<h1>Hello, World!</h1>")
}

pub async fn create_shortlink(
    Json(req): Json<dto::CreateShortLinkReq>,
    Extension(pool): Extension<Pool<MySql>>,
) -> impl IntoResponse {
    println!("req:=======>{:#?}", req);
    match models::create_shortlink(&pool, &req.url).await {
        Ok(_) => {
            (StatusCode::OK, Json(dto::CreateShortLinkResp {
                ok: true
            }))
        }
        Err(_) => {
            (StatusCode::INTERNAL_SERVER_ERROR, Json(dto::CreateShortLinkResp {
                ok: false
            }))
        }
    }
}

pub async fn delete_shortlink(
    Json(req): Json<dto::DeleteShortLinkReq>,
    Extension(pool): Extension<Pool<MySql>>,
) -> impl IntoResponse {
    println!("{:?}", req);
    match models::delete_shortlink(&pool, req.id).await {
        Ok(_) => {
            (StatusCode::OK, Json(dto::DeleteShortLinkResp {
                ok: true
            }))
        }
        Err(_) => {
            (StatusCode::INTERNAL_SERVER_ERROR, Json(dto::DeleteShortLinkResp {
                ok: false
            }))
        }
    }
}