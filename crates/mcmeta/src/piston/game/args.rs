use serde_either::StringOrStruct;

use super::cond::ConditionalValue;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameArgs {
    pub game: Vec<StringOrStruct<ConditionalValue>>,

    #[serde(default)]
    pub jvm: Vec<StringOrStruct<ConditionalValue>>,
}

impl GameArgs {
    pub fn get_game_args(&self, feats: &Vec<String>) -> Vec<String> {
        let mut args = Vec::new();

        for item in &self.game {
            let mut arg = match item {
                StringOrStruct::String(val) => vec![val.clone()],
                StringOrStruct::Struct(val) => val.value(feats),
            };

            args.append(&mut arg);
        }

        args
    }

    pub fn get_jvm_args(&self, feats: &Vec<String>) -> Vec<String> {
        let mut args = Vec::new();

        for item in &self.jvm {
            let mut arg = match item {
                StringOrStruct::String(val) => vec![val.clone()],
                StringOrStruct::Struct(val) => val.value(feats),
            };

            args.append(&mut arg);
        }

        args
    }
}
