#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JavaRef {
    pub component: String,
    pub major_version: i32,
}
