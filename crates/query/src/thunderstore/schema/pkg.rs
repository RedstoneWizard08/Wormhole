// MIT License
//
// Copyright (c) 2024 Jacob Sapoznikow <jacob1coder@gmail.com>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE. 

use super::{
    list::CommunityListing,
    ver::{NewPackageVersion, PackageVersion},
};

#[derive(
    Serialize, Deserialize, Type, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default,
)]
pub struct PackageListing {
    pub name: String,
    pub full_name: String,
    pub owner: String,
    pub package_url: String,
    pub donation_link: Option<String>,
    pub date_created: String,
    pub date_updated: String,
    pub uuid4: String,
    pub rating_score: i32,
    pub is_pinned: bool,
    pub is_deprecated: bool,
    pub has_nsfw_content: bool,
    pub categories: Vec<String>,
    pub versions: Vec<PackageVersion>,
}

#[derive(
    Serialize, Deserialize, Type, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default,
)]
pub struct Package {
    pub namespace: String,
    pub name: String,
    pub full_name: String,
    pub owner: String,
    pub package_url: String,
    pub date_created: String,
    pub date_updated: String,
    pub rating_score: i32,
    pub is_pinned: bool,
    pub is_deprecated: bool,
    pub total_downloads: i32,
    pub latest: NewPackageVersion,
    pub community_listings: Vec<CommunityListing>,
}
