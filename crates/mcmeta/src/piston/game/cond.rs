use serde_either::StringOrStruct;

use super::rules::Rule;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConditionalValue {
    pub rules: Vec<Rule>,
    pub value: StringOrStruct<Vec<String>>,
}

impl ConditionalValue {
    pub fn value(&self, features: &Vec<String>) -> Vec<String> {
        if self.rules.iter().all(|rule| rule.should_apply(features)) {
            match &self.value {
                StringOrStruct::String(val) => vec![val.clone()],
                StringOrStruct::Struct(val) => val.clone(),
            }
        } else {
            vec![]
        }
    }
}
