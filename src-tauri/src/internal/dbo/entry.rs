use serde::Serialize;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Serialize, Debug)] // Debug for printing to console
pub enum FeedEntryType {
    RSS(RssEntry),
    ATOM(AtomEntry),
}

#[derive(Serialize, Debug)] // Debug for printing to console
pub struct RssEntry {
    // Required fields
    pub title: String,
    pub hash: i64,
    pub link: Option<String>,
    pub description: Option<String>,
    // Optional fields
    pub pub_date: Option<String>,
    pub author: Option<String>,
    pub category: Option<String>,
    pub comments: Option<String>,
    pub enclosure: Option<String>,
    pub guid: Option<String>,
}

impl RssEntry {
    pub fn new(
        title: String,
        link: Option<String>,
        description: Option<String>,
        pub_date: Option<String>,
        author: Option<String>,
        category: Option<String>,
        comments: Option<String>,
        enclosure: Option<String>,
        guid: Option<String>,
    ) -> Self {
        let mut hasher = DefaultHasher::new();
        title.hash(&mut hasher);
        link.hash(&mut hasher);
        description.hash(&mut hasher);
        pub_date.hash(&mut hasher);
        author.hash(&mut hasher);
        category.hash(&mut hasher);
        comments.hash(&mut hasher);
        enclosure.hash(&mut hasher);
        guid.hash(&mut hasher);
        let hash = hasher.finish() as i64;

        RssEntry {
            title,
            link,
            description,
            pub_date,
            author,
            category,
            comments,
            enclosure,
            guid,
            hash,
        }
    }
}

// Struct for Atom item
#[derive(Serialize, Debug)] // Debug for printing to console
pub struct AtomEntry {
    // Required fields
    pub title: String,
    pub link: Option<String>,
    pub summary: Option<String>,
    // Optional fields
    pub id: Option<String>,
    pub updated: Option<String>,
    pub author: Option<String>,
    pub category: Option<String>,
    pub content: Option<String>,
    pub contributor: Option<String>,
    pub pub_date: Option<String>,
    pub rights: Option<String>,
    pub hash: i64,
}

impl AtomEntry {
    pub fn new(
        title: String,
        link: Option<String>,
        summary: Option<String>,
        id: Option<String>,
        updated: Option<String>,
        author: Option<String>,
        category: Option<String>,
        content: Option<String>,
        contributor: Option<String>,
        pub_date: Option<String>,
        rights: Option<String>,
    ) -> Self {
        let mut hasher = DefaultHasher::new();

        // Hash the title
        title.hash(&mut hasher);

        // Hash each optional field if it exists
        let fields_to_hash = [
            &link,
            &summary,
            &id,
            &updated,
            &author,
            &category,
            &content,
            &contributor,
            &pub_date,
            &rights,
        ];

        for field in fields_to_hash.iter() {
            if let Some(value) = field {
                value.hash(&mut hasher);
            }
        }

        let hash = hasher.finish() as i64;

        AtomEntry {
            title,
            link,
            summary,
            id,
            updated,
            author,
            category,
            content,
            contributor,
            pub_date,
            rights,
            hash,
        }
    }
}
