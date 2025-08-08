use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Category {
    pub name: String,
    pub category_weekly_quota: u32, // in hours
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag {
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub id: String,
    pub title: String,
    pub category: String,
    pub tags: Vec<String>,
    pub start: DateTime<Utc>,
    pub end: Option<DateTime<Utc>>,
    pub duration: u32, // in minutes
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MetronData {
    pub categories: Vec<Category>,
    pub tags: Vec<Tag>,
    pub sessions: Vec<Session>,
    pub total_weekly_quota: Option<u32>, // in hours
}

impl MetronData {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Debug)]
pub enum MetronError {
    CategoryNotFound,
    TagNotFound,
    SessionNotFound,
    QuotaExceeded,
    InvalidDuration,
    TagLimitExceeded,
    DuplicateName,
    StorageError(String),
}

impl std::fmt::Display for MetronError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MetronError::CategoryNotFound => write!(f, "Category not found"),
            MetronError::TagNotFound => write!(f, "Tag not found"),
            MetronError::SessionNotFound => write!(f, "Session not found"),
            MetronError::QuotaExceeded => write!(f, "Weekly quota would be exceeded"),
            MetronError::InvalidDuration => write!(f, "Duration must be a multiple of 15 minutes"),
            MetronError::TagLimitExceeded => write!(f, "Maximum of 7 tags allowed"),
            MetronError::DuplicateName => write!(f, "Name already exists"),
            MetronError::StorageError(msg) => write!(f, "Storage error: {}", msg),
        }
    }
}

impl std::error::Error for MetronError {}

pub type Result<T> = std::result::Result<T, MetronError>;
