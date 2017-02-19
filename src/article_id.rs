extern crate uuid;
extern crate base32;
extern crate regex;

use self::uuid::Uuid;
use self::base32::Alphabet;
use self::regex::Regex;
use std::path::Path;

const BASE32_ALPHABET: Alphabet = Alphabet::RFC4648 { padding: false };

pub struct ArticleId {
    pub id: String
}

impl ArticleId {
    pub fn generate_draft_id() -> ArticleId {
        let uuid = Uuid::new_v4();
        ArticleId::new(&base32::encode(BASE32_ALPHABET, uuid.as_bytes()))
    }

    pub fn new(id: &str) -> ArticleId {
        ArticleId { id: id.to_string() }
    }

    pub fn from_draft_dir(path: &Path) -> Option<ArticleId> {
        from_article_dir(path, validate_draft_id)
    }

    pub fn from_post_dir(path: &Path) -> Option<ArticleId> {
        from_article_dir(path, validate_post_id)
    }
}

fn from_article_dir<F>(path: &Path, validator: F) -> Option<ArticleId>
        where F: Fn(&str) -> Option<&str> {
    path.iter().last()
        .and_then(|os_str| os_str.to_str())
        .and_then(validator)
        .map(ArticleId::new)
}

fn validate_draft_id(id: &str) -> Option<&str> {
    base32::decode(BASE32_ALPHABET, id)
        .and_then(|uuid| if uuid.len() == 16 { Some(id) } else { None })
}

fn validate_post_id(id: &str) -> Option<&str> {
    let regex = Regex::new(r"[0-9a-f]+").unwrap(); // Error means a bug.
    if regex.is_match(id) { Some(id) } else { None }
}

