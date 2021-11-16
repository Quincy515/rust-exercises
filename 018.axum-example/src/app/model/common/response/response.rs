#![allow(dead_code)]
use std::convert::Infallible;

use axum::{
    body::{Bytes, Full},
    http::{header, HeaderValue, StatusCode},
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
pub struct Response<T: Serialize> {
    pub code: ResponseCode,
    pub data: T,
    pub msg: String,
}

/// Err-服务错误返回, Privilege-权限限制, Auth-需要认证
#[derive(Deserialize, Serialize, Debug)]
pub enum ResponseCode {
    Success = 0,
    Error = 7,
    Privilege = 8,
    Auth = 9,
}

impl<T: Serialize + Default> Response<T> {
    pub fn new(code: ResponseCode, data: T, msg: String) -> Response<T> {
        Response { code, data, msg }
    }
    pub fn ok() -> Response<T> {
        Response::new(ResponseCode::Success, T::default(), "操作成功".to_string())
    }
    pub fn ok_with_msg(msg: String) -> Response<T> {
        Response::new(ResponseCode::Success, T::default(), msg)
    }
    pub fn ok_with_data(data: T) -> Response<T> {
        Response::new(ResponseCode::Success, data, "操作成功".to_string())
    }
    pub fn ok_with_detailed(data: T, msg: String) -> Response<T> {
        Response::new(ResponseCode::Success, data, msg)
    }
    pub fn fail() -> Response<T> {
        Response::new(ResponseCode::Error, T::default(), "操作失败".to_string())
    }
    pub fn fail_with_msg(msg: String) -> Response<T> {
        Response::new(ResponseCode::Error, T::default(), msg)
    }
    pub fn fail_with_detailed(data: T, msg: String) -> Response<T> {
        Response::new(ResponseCode::Error, data, msg)
    }
}

impl<T: Serialize + Default> IntoResponse for Response<T> {
    type Body = Full<Bytes>;

    type BodyError = Infallible;

    fn into_response(self) -> axum::http::Response<Self::Body> {
        let bytes = match serde_json::to_vec(&self) {
            Ok(res) => res,
            Err(err) => {
                println!("{}", err);
                return axum::http::Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .header(header::CONTENT_TYPE, "text/plain")
                    .body(Full::from(err.to_string()))
                    .unwrap();
            }
        };

        let mut res = axum::http::Response::new(Full::from(bytes));
        res.headers_mut().insert(
            header::CONTENT_TYPE,
            HeaderValue::from_static("application/json"),
        );
        res
    }
}
