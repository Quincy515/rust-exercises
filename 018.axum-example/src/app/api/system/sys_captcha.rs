use axum::{extract::Extension, Json};
use captcha::{filters::Noise, Captcha};
use serde_json::{Value, json};

use crate::{
    app::model::{
        common::response::response::Response, system::response::sys_captcha::SysCaptchaResponse,
    },
    config::databases::Pool,
};

pub async fn captcha(Extension(_pool): Extension<Pool>) -> Json<Value> {
    println!("captcha");
    let mut c = Captcha::new();
    c.add_chars(6).apply_filter(Noise::new(0.7)).view(220, 120);
    let s = c.as_base64().expect("Error.");
    let res = Response::ok_with_detailed(
        SysCaptchaResponse {
            captcha_id: c.chars_as_string(),
            pic_path: format!("data:image/png;base64,{}", s),
            captcha_length: 6,
        },
        "验证码获取成功".to_string(),
    );
    Json(json!(res))
}
