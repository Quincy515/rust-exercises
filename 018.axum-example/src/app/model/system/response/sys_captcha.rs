use serde::Serialize;

#[derive(Default, Debug, Serialize)]
pub struct SysCaptchaResponse {
    pub captcha_id: String,
    pub pic_path: String,
    pub captcha_length: i32,
}
