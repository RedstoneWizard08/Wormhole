use std::collections::HashMap;

use serde_either::StringOrStruct;

use super::system::{Arch, OperatingSystem};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conditional {
    pub rules: Vec<Rule>,
    pub value: StringOrStruct<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct Rule {
    #[serde(default)]
    pub action: RuleAction,
    #[serde(default)]
    pub features: HashMap<String, bool>,
    #[serde(default)]
    pub os: OsRule,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type, Default)]
pub struct OsRule {
    #[serde(default)]
    pub name: OperatingSystem,
    #[serde(default)]
    pub arch: Arch,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Type, Default)]
#[serde(rename_all = "snake_case")]
pub enum RuleAction {
    #[default]
    Allow,
    Disallow,
}
