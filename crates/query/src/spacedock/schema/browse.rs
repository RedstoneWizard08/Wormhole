use crate::{mod_::Mod, source::Paginated, IntoAsync};

use super::info::ModInfo;

#[derive(
    Serialize, Deserialize, Type, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default,
)]
pub struct BrowseResult {
    pub result: Option<Vec<ModInfo>>,
    pub count: Option<i32>,
    pub pages: Option<i32>,
    pub page: Option<i32>,
}

impl BrowseResult {
    pub fn finish(&self) -> Self {
        let mut out = BrowseResult::default();
        let mut _res = Vec::new();

        if let Some(result) = self.result.clone() {
            _res = result.iter().map(|v| v.finish(true)).collect();
        }

        out.result = Some(_res);
        out.count = Some(self.count.unwrap_or(0));
        out.pages = Some(self.pages.unwrap_or(0));
        out.page = Some(self.page.unwrap_or(0));

        out
    }
}

#[async_trait]
impl IntoAsync<Paginated<Mod>> for BrowseResult {
    async fn into_async(self) -> Paginated<Mod> {
        Paginated {
            data: self.result.unwrap_or_default().into_async().await,
            page: self.page,
            per_page: self.count,
            pages: self.pages,
        }
    }
}
