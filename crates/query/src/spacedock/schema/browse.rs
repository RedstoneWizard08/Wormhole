// MIT License
//
// Copyright (c) 2024 Jacob Sapoznikow <jacob1coder@gmail.com>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE. 

use crate::{mod_::Mod, source::Paginated};

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
            pages: val.pages,
        }
    }
}
