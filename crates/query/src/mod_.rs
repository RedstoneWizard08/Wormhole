// MIT License
//
// Copyright (c) 2024 Jacob Sapoznikow <jacob1coder@gmail.com>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE. 

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Type)]
pub struct Mod {
    /// The mod's ID in its source.
    /// This could be an integer or a string,
    /// and since we support multiple platforms,
    /// a string is the most flexible.
    pub id: String,

    /// The game ID.
    pub game_id: Option<i32>,

    /// The mod's versions.
    pub versions: Vec<ModVersion>,

    /// The mod's name.
    pub name: String,

    /// Where the mod came from.
    /// This is a reference to a source in the database.
    pub source: i32,

    /// The mod's icon.
    pub icon: Option<String>,
}

unsafe impl Send for Mod {}
unsafe impl Sync for Mod {}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Type)]
pub struct ModVersion {
    /// The version ID.
    pub id: String,

    /// The version name. Some sources may not have this.
    pub name: Option<String>,

    /// The file name.
    pub file_name: Option<String>,

    /// The size in bytes of the file.
    pub size: Option<String>,

    /// The SHA-512 hash of the file.
    pub hash: Option<String>,

    /// The download URL.
    pub url: Option<String>,
}

unsafe impl Send for ModVersion {}
unsafe impl Sync for ModVersion {}

impl ModVersion {
    pub fn file_name(&self) -> String {
        self.file_name.clone().unwrap_or_else(|| {
            self.url
                .clone()
                .unwrap()
                .split('/')
                .last()
                .unwrap()
                .to_string()
        })
    }
}
