use std::collections::HashMap;

use serde_either::StringOrStruct;

use super::rules::Conditional;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Arguments {
    #[serde(default)]
    pub game: Vec<StringOrStruct<Conditional>>,
    #[serde(default)]
    pub jvm: Vec<StringOrStruct<Conditional>>,
}

impl Arguments {
    pub fn extend(&mut self, other: Arguments) {
        self.game.extend(other.game);
        self.jvm.extend(other.jvm);
    }

    pub fn game_args(&self, features: &HashMap<String, bool>) -> Vec<String> {
        let mut args = Vec::new();

        for item in &self.game {
            match item {
                StringOrStruct::String(s) => args.push(s.clone()),
                StringOrStruct::Struct(v) => args.extend(v.value(features)),
            }
        }

        args
    }

    pub fn jvm_args(&self, features: &HashMap<String, bool>) -> Vec<String> {
        let mut args = Vec::new();

        for item in &self.jvm {
            match item {
                StringOrStruct::String(s) => args.push(s.clone()),
                StringOrStruct::Struct(v) => args.extend(v.value(features)),
            }
        }

        args
    }
}
