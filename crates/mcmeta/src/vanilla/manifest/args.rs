use serde_either::StringOrStruct;

use super::rules::Conditional;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Arguments {
    pub game: Vec<StringOrStruct<Conditional>>,
    pub jvm: Vec<StringOrStruct<Conditional>>,
}
