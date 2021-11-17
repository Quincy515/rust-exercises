use axum::{extract::Extension, Json};
use captcha::{filters::Noise, Captcha};
use clap::Parser;
use serde_json::{json, Value};

use crate::{
    app::model::{
        common::response::response::Response, system::response::sys_captcha::SysCaptchaResponse,
    },
    config::{self, databases::Pool},
};

// 生成验证码
pub async fn captcha(Extension(_pool): Extension<Pool>) -> Json<Value> {
    let config = config::env::CaptchaConfig::parse();

    let mut c = Captcha::new();
    c.add_chars(config.length)
        .apply_filter(Noise::new(config.noise))
        .view(config.width, config.height);
    let s = c.as_base64().expect("gen captcha failed.");
    let id = md5::compute(c.chars_as_string().to_lowercase().as_bytes());

    let res = Response::ok_with_detailed(
        SysCaptchaResponse {
            captcha_id: format!("{:x}", id),
            pic_path: format!("data:image/png;base64,{}", s),
            captcha_length: config.length as i32,
        },
        "验证码获取成功".to_string(),
    );
    Json(json!(res))
}
