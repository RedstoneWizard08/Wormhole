//! The progress API.

/// A progress payload.
#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize, Type,
)]
pub struct ProgressPayload {
    /// The total number of iterations. For a request or
    /// file operation, this will be the total size of
    /// the file, in bytes.
    pub total: u64,

    /// The current processed iterations count. For a
    /// request or file operation, this will be how many
    /// bytes have been received/processed/written.
    pub current: u64,

    /// The name of the file or operation being done.
    pub name: String,
}
