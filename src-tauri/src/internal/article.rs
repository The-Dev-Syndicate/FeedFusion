use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Article {
    pub title: String,
    pub description: String,
    pub author: String,
    pub datetime: String,
}

impl Article {
    pub fn new(title: &str, description: &str, author: &str, datetime: &str) -> Self {
        Self {
            title: title.to_string(),
            description: description.to_string(),
            author: author.to_string(),
            datetime: datetime.to_string(),
        }
    }
}