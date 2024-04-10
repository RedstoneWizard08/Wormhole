use crate::{mod_::Mod, source::Paginated};

use super::info::ModInfo;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
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

impl From<BrowseResult> for Paginated<Mod> {
    fn from(val: BrowseResult) -> Self {
        Self {
            data: val
                .result
                .unwrap_or_default()
                .iter()
                .cloned()
                .map(|v| v.into())
                .collect::<Vec<_>>(),
            page: val.page,
            per_page: val.count,
        }
    }
}
