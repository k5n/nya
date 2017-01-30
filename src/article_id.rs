extern crate uuid;
extern crate base32;

use self::uuid::Uuid;
use self::base32::Alphabet;
use std::path::Path;

const BASE32_ALPHABET: Alphabet = Alphabet::RFC4648 { padding: false };

pub struct ArticleId {
    pub id: String
}

impl ArticleId {
    pub fn generate() -> ArticleId {
        let uuid = Uuid::new_v4();
        ArticleId::new(&base32::encode(BASE32_ALPHABET, uuid.as_bytes()))
    }

    pub fn new(id: &str) -> ArticleId {
        ArticleId { id: id.to_string() }
    }

    pub fn from_article_dir(path: &Path) -> Option<ArticleId> {
        if path.is_dir() {
            path.iter().last()
                .and_then(|os_str| os_str.to_str())
                .and_then(validate)
                .map(ArticleId::new)
        } else {
            None
        }
    }

}

fn validate(id: &str) -> Option<&str> {
    base32::decode(BASE32_ALPHABET, id)
        .and_then(|uuid| if uuid.len() == 16 { Some(id) } else { None })
}

