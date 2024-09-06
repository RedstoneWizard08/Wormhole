use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct ProjectVersion {
    /// The ID of the version.
    pub id: String,

    /// The project ID that this version belongs to.
    pub project_id: String,

    /// The source the version can be found in. See [`crate::models::QueryClient::id`].
    pub source_id: String,

    /// The name of this version.
    pub name: String,

    /// The version number, if it differs from the name.
    pub version_number: Option<String>,

    /// The URL to find this version on the source's website.
    pub url: String,

    /// The version's author. This account uploaded the version.
    pub author: Option<String>,

    /// The file name for this version.
    pub file: String,

    /// The URL to download this version from (if it's included from the API).
    pub download_url: Option<String>,

    /// How many downloads this version has.
    pub downloads: Option<u64>,

    /// This version's changelog.
    pub changelog: Option<String>,

    /// When this version was uploaded.
    pub uploaded: Option<DateTime<Utc>>,

    /// The release type.
    pub kind: VersionKind,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Type)]
pub enum VersionKind {
    Alpha,
    Beta,
    Release,
    Prerelease,
}
