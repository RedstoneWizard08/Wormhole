#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Paginated<T> {
    pub total_pages: u64,
    pub current_page: u64,
    pub items_per_page: u64,
    pub data: Vec<T>,
}
