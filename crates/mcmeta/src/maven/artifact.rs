#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct Artifact {
    pub package: String,
    pub artifact: String,
    pub version: Option<String>,
    pub classifier: Option<String>,
}

impl Artifact {
    pub fn new(data: impl AsRef<str>) -> Self {
        let data = data.as_ref();
        let mut data = data.split(":");
        let package = data.next().unwrap().into();
        let artifact = data.next().unwrap().into();
        let version = data.next().map(String::from);
        let classifier = data.next().map(String::from);

        Self {
            package,
            artifact,
            version,
            classifier,
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
        self.to_string().replace(":", "/").replace(".", "/")
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
        let it = self.clone().to_string();
        let mut s = format!("{}-{}", self.artifact, self.version.unwrap());

        if let Some(cls) = self.classifier {
            s.push_str(&format!("-{}", cls));
        }

        format!("{}/{}.jar", it, s)
    }
}

impl<T: AsRef<str>> From<T> for Artifact {
    fn from(value: T) -> Self {
        Self::new(value)
    }
}
