#[derive(
    Serialize, Deserialize, Type, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default,
)]
pub struct PackageVersion {
    pub name: String,
    pub full_name: String,
    pub description: String,
    pub icon: String,
    pub version_number: String,
    pub dependencies: Vec<String>,
    pub download_url: String,
    pub downloads: i32,
    pub date_created: String,
    pub website_url: Option<String>,
    pub is_active: bool,
    pub uuid4: String,
    pub file_size: i32,
}

#[derive(
    Serialize, Deserialize, Type, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default,
)]
pub struct NewPackageVersion {
    pub namespace: String,
    pub name: String,
    pub version_number: String,
    pub full_name: String,
    pub description: String,
    pub icon: String,
    pub dependencies: Vec<String>,
    pub download_url: String,
    pub downloads: i32,
    pub date_created: String,
    pub website_url: Option<String>,
    pub is_active: bool,
}
