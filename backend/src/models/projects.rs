use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Projects {
    pub id: Uuid,
    pub slug: String,
    pub title: String,
    pub short_description: Option<String>,
    pub cover_image_url: Option<String>,
    pub repo_url: Option<String>,
    pub live_url: Option<String>,
    pub tech_stack: Option<Vec<String>>,
    pub is_published: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct ProjectMarkdown {
    pub project_id: Uuid,
    pub content_md: String,
}
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct ProjectDetail {
    pub id: Uuid,
    pub slug: String,
    pub title: String,
    pub short_description: Option<String>,
    pub cover_image_url: Option<String>,
    pub repo_url: Option<String>,
    pub live_url: Option<String>,
    pub tech_stack: Option<Vec<String>>,
    pub content_md: String,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct PostProject {
    pub slug: String,
    pub title: String,
    pub short_description: Option<String>,
    pub cover_image_url: Option<String>,
    pub repo_url: Option<String>,
    pub live_url: Option<String>,
    pub tech_stack: Option<Vec<String>>,
    pub content_md: String,
}
