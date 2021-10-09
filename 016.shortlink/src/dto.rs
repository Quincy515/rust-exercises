use serde::{Deserialize, Serialize};
use crate::models;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateShortLinkReq {
    pub url: String
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateShortLinkResp {
    pub ok: bool
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeleteShortLinkReq {
    pub id: u64
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeleteShortLinkResp {
    pub ok: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetShortLinkReq {
    pub id: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetShortLinkResp {
    pub shortlink: models::ShortLink,
}