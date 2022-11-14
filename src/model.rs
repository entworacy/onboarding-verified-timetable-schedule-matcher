

use std::error::Error;
use std::fmt::{Display, Formatter};
use serde::{Serialize, Deserialize};



#[derive(Debug, Serialize, Deserialize)]
pub struct NijisanjiResponse {
    status: String,
    data: NijisanjiData
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NijisanjiData {
    events: Vec<Event>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Event {
    pub name: String,
    pub description: String,
    pub url: String,
    pub thumbnail: String,
    pub start_date: String,
    pub livers: Vec<NijisanjiLiver>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NijisanjiLiver {
    name: String,
    avatar: String
}

// Holo
#[derive(Debug, Serialize, Deserialize)]
pub struct DateGroupList {
    pub dateGroupList: Vec<DateGroup>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DateGroup {
    pub displayDate: String,
    pub datetime: String,
    pub videoList: Vec<Video>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Video {
    pub displayDate: String,
    pub datetime: String,
    pub isLive: bool,
    pub platformType: i32,
    pub url: String,
    pub thumbnail: String,
    pub title: String,
    pub name: String,
    pub talent: Talent,
    pub collaboTalents: Vec<Talent>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Talent {
    pub iconImageUrl: String
}

// Common

#[derive(Debug, Serialize, Deserialize)]
pub enum PlatformType {
    Youtube = 0, Twitch = 1, Other = 2
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CommonSchedule {
    pub platform_type: u32,
    pub contents_title: String,
    pub contents_author_name: String,
    pub contents_author_profile_url: String,
    pub start_date: String,
    pub url: String,
    pub contents_thumbnail: Option<String>,
    pub has_collaborator: bool,
    pub collaborators: Vec<CommonCollaborator>,
    pub expected_streamer_language: Vec<ExpectedLanguageInfo>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CommonCollaborator {
    pub collaborator_name: Option<String>,
    pub collaborator_profile_url: String
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExpectedLanguageInfo {
    pub expected_target: String,
    pub expected_language: Option<String>,
    pub accuracy: f64
}

// Error

#[derive(Debug)]
pub struct HttpError {
    pub message: String,
    pub http_status_code: i32
}

impl HttpError {
    pub fn new(
        message: &str,
        http_status_code: u16
    ) -> HttpError {
        HttpError { message: message.to_string(), http_status_code: http_status_code.clone() as i32 }
    }
}

impl Display for HttpError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[STATUS: {}] {}", self.http_status_code, self.message)
    }
}

impl Error for HttpError {
    fn description(&self) -> &str {
        &self.message
    }
}