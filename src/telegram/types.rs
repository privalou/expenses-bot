use std::fmt::Result;

use serde::Serialize;

#[derive(Serialize, Debug, Default)]
pub struct Message<'a> {
    pub chat_id: &'a str,
    pub text: &'a str,
    pub disable_notification: bool,
    pub disable_web_page_preview: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<&'a ReplyMarkup>,
}

#[derive(Serialize, Debug, Default)]
pub struct EditMessage<'a> {
    pub chat_id: &'a str,
    pub message_id: &'a str,
    pub text: &'a str,
    pub disable_notification: bool,
    pub disable_web_page_preview: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<&'a ReplyMarkup>,
}

#[derive(Serialize)]
#[serde(untagged)]
#[allow(dead_code)]
pub enum ReplyMarkup {
    InlineKeyboardMarkup(InlineKeyboardMarkup),
}

impl std::fmt::Debug for ReplyMarkup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result {
        f.debug_struct("ReplyMarkup")
            .field("InlineKeyboardMarkup", &"foo")
            .finish()
    }
}

#[derive(Serialize, Debug, Default, Clone)]
pub struct InlineKeyboardButton {
    pub text: String,
    pub callback_data: String,
}

impl InlineKeyboardButton {
    #[allow(dead_code)]
    pub fn new(text: &str) -> Self {
        InlineKeyboardButton {
            text: text.to_string(),
            callback_data: text.to_string(),
        }
    }
}

#[allow(dead_code)]
#[derive(Serialize, Debug, Default)]
pub struct InlineKeyboardMarkup {
    pub inline_keyboard: Vec<Vec<InlineKeyboardButton>>,
}

#[derive(Serialize, Debug, Default)]
pub struct Image<'a> {
    pub chat_id: &'a str,
    pub photo: &'a str,
    pub disable_notification: bool,
}

#[derive(Serialize, Debug, Default)]
pub struct EditImage<'a> {
    pub chat_id: &'a str,
    pub message_id: &'a str,
    pub photo: &'a str,
    pub disable_notification: bool,
    pub media: Media<'a>,
}

#[derive(Serialize, Debug, Default)]
pub struct Media<'a> {
    #[serde(rename = "type")]
    pub type_: &'a str,
}
