use serde::{Deserialize, Serialize};
use serde_json::Value;

pub type MessageStruct = Vec<Root2>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root2 {
    pub id: String,
    #[serde(rename = "type")]
    pub type_field: i64,
    pub content: String,
    #[serde(rename = "channel_id")]
    pub channel_id: String,
    pub author: Author,
    pub attachments: Option<Vec<Value>>,
    pub embeds: Option<Vec<Embed>>,
    pub mentions: Vec<Value>,
    #[serde(rename = "mention_roles")]
    pub mention_roles: Vec<Value>,
    pub pinned: bool,
    #[serde(rename = "mention_everyone")]
    pub mention_everyone: bool,
    pub tts: bool,
    pub timestamp: String,
    #[serde(rename = "edited_timestamp")]
    pub edited_timestamp: Value,
    pub flags: i64,
    pub components: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Author {
    pub id: String,
    pub username: String,
    pub avatar: Option<String>,
    pub discriminator: String,
    #[serde(rename = "public_flags")]
    pub public_flags: i64,
    pub flags: i64,
    pub banner: Value,
    #[serde(rename = "accent_color")]
    pub accent_color: Value,
    #[serde(rename = "global_name")]
    pub global_name: Option<String>,
    #[serde(rename = "avatar_decoration_data")]
    pub avatar_decoration_data: Value,
    #[serde(rename = "banner_color")]
    pub banner_color: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Embed {
    #[serde(rename = "type")]
    pub type_field: String,
    pub url: Option<String>,
    pub provider: Option<Provider>,
    pub thumbnail: Option<Thumbnail>,
    pub video: Option<Video>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Provider {
    pub name: String,
    pub url: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Thumbnail {
    pub url: Option<String>,
    #[serde(rename = "proxy_url")]
    pub proxy_url: Option<String>,
    pub width: i64,
    pub height: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Video {
    pub url: Option<String>,
    #[serde(rename = "proxy_url")]
    pub proxy_url: Option<String>,
    pub width: i64,
    pub height: i64,
}