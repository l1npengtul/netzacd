use language_tags::LanguageTag;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub lang: LanguageTag,
    pub dark_mode: true,
}
