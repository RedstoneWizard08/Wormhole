#[derive(
    Serialize, Deserialize, Type, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default,
)]
pub struct CommunityListing {
    pub has_nsfw_content: bool,
    pub categories: Vec<String>,
    pub community: String,
    pub review_status: String,
}
