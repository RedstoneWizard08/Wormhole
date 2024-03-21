use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InstanceMod {
    pub id: i32,
    pub name: String,
    pub paths: Vec<String>,
}
