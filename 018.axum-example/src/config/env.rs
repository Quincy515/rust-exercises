use std::{env, net::IpAddr};

use clap::Parser;

lazy_static! {
    pub static ref JWT_SECRET: String = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
}

#[derive(Debug, Parser)]
pub struct ServerConfig {
    #[clap(default_value = "127.0.0.1", env)]
    pub host: IpAddr,
    #[clap(default_value = "3000", env)]
    pub port: u16,
}

#[derive(Debug, Parser)]
pub struct CaptchaConfig {
    #[clap(default_value = "4", env = "CAPTCHA_LONG")]
    pub length: u32,
    #[clap(default_value = "240", env = "CAPTCHA_WIDTH")]
    pub width: u32,
    #[clap(default_value = "80", env = "CAPTCHA_HEIGHT")]
    pub height: u32,
    #[clap(default_value = "0.5", env = "CAPTCHA_NOISE")]
    pub noise: f32,
}


#[derive(Debug, Parser)]
pub struct JwtConfig {
    #[clap(default_value = "qmPlus", env = "SIGNING_KEY")]
    pub signing_key: String,
    #[clap(default_value = "604800", env = "EXPIRES_TIME")]
    pub expires_time: i64,
    #[clap(default_value = "86400", env = "BUFFER_TIME")]
    pub buffer_time: i64,
    #[clap(default_value = "qmPlus", env = "ISSUER")]
    pub issuer: String,
}