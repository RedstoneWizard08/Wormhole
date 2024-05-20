// MIT License
//
// Copyright (c) 2024 Jacob Sapoznikow <jacob1coder@gmail.com>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE. 

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MavenCoordinate {
    pub group: String,
    pub artifact: String,
    pub version: Option<String>,
    pub classifier: Option<String>,
    pub ext: String,
}

impl MavenCoordinate {
    pub fn group_path(&self) -> String {
        self.group.replace(".", "/")
    }

    pub fn artifact(&self) -> String {
        let mut name = format!("{}-{}", self.artifact, self.version.clone().unwrap());

        if let Some(classifier) = &self.classifier {
            name.push_str(&format!("-{}", classifier));
        }

        name
    }

    pub fn jar(&self) -> String {
        format!("{}.{}", self.artifact(), self.ext)
    }

    pub fn path(&self) -> String {
        let mut path = format!("{}/{}", self.group_path(), self.artifact);

        if let Some(ver) = &self.version {
            path.push_str(&format!("/{}", ver));
        }

        format!("{}/{}.{}", path, self.artifact(), self.ext)
    }

    pub fn url(&self, repo: impl AsRef<str>) -> String {
        format!("{}/{}", repo.as_ref(), self.path(),)
    }
}

impl<T: AsRef<str>> From<T> for MavenCoordinate {
    fn from(value: T) -> Self {
        let val = value.as_ref();
        let mut ext_spl = val.split('@');
        let mut spl = ext_spl.next().unwrap().split(':');
        let group = spl.next().unwrap().to_string();
        let artifact = spl.next().unwrap().to_string();
        let version = spl.next().map(|v| v.to_string());
        let classifier = spl.next().map(|v| v.to_string());
        let ext = ext_spl.next().unwrap_or("jar").to_string();

        Self {
            group,
            artifact,
            version,
            classifier,
            ext,
        }
    }
}
