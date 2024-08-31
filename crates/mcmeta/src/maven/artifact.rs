#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct Artifact {
    pub package: String,
    pub artifact: String,
    pub version: Option<String>,
    pub classifier: Option<String>,
    pub ext: String,
}

impl Artifact {
    pub fn new(data: impl AsRef<str>) -> Self {
        let data = data.as_ref();
        let raw = data.split("@").collect::<Vec<_>>();
        let mut data = raw.get(0).unwrap().split(":");

        let ext = if raw.len() > 1 {
            raw.get(1).unwrap().to_string()
        } else {
            "jar".into()
        };

        let package = data.next().unwrap().into();
        let artifact = data.next().unwrap().into();
        let version = data.next().map(String::from);
        let classifier = data.next().map(String::from);

        Self {
            package,
            artifact,
            version,
            classifier,
            ext,
        }
    }

    pub fn to_string(self) -> String {
        let mut s = format!("{}:{}", self.package, self.artifact);

        if let Some(ver) = self.version {
            s.push_str(&format!(":{}", ver));
        }

        if let Some(cls) = self.classifier {
            s.push_str(&format!(":{}", cls));
        }

        s
    }

    pub fn to_string_versionless(self) -> String {
        format!("{}:{}", self.package, self.artifact)
    }

    pub fn base_url(self) -> String {
        format!(
            "{}/{}/{}",
            self.package.replace(".", "/"),
            self.artifact,
            self.version.unwrap()
        )
    }

    pub fn base_url_versionless(self) -> String {
        self.to_string_versionless()
            .replace(":", "/")
            .replace(".", "/")
    }

    pub fn set_version(mut self, ver: impl AsRef<str>) -> Self {
        self.version = Some(ver.as_ref().into());
        self
    }

    pub fn set_classifier(mut self, cls: impl AsRef<str>) -> Self {
        self.classifier = Some(cls.as_ref().into());
        self
    }

    pub fn url(self) -> String {
        let it = self.clone().base_url();
        let mut s = format!("{}-{}", self.artifact, self.version.unwrap());

        if let Some(cls) = self.classifier {
            s.push_str(&format!("-{}", cls));
        }

        format!("{}/{}.{}", it, s, self.ext)
    }
}

impl<T: AsRef<str>> From<T> for Artifact {
    fn from(value: T) -> Self {
        Self::new(value)
    }
}
