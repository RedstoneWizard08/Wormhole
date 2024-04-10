use furse::structures::mod_structs::Mod;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QueryResult {
    pub data: Vec<Mod>,
    pub pagination: QueryPagination,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(rename_all = "camelCase")]
pub struct QueryPagination {
    pub index: i32,
    pub page_size: i32,
    pub result_count: i32,
    pub total_count: i32,
}
