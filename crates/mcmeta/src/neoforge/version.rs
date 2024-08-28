#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct NeoForgeVersion {
    pub minecraft: String,
    pub neoforge: i32,
    pub beta: bool,
}

impl NeoForgeVersion {
    pub fn new(raw: impl AsRef<str>) -> Self {
        let mut data = raw.as_ref().split(".");
        let minecraft = format!("1.{}.{}", data.next().unwrap(), data.next().unwrap());
        let neo = data.next().unwrap();
        let mut neo = neo.split("-");
        let neoforge = neo.next().unwrap().parse::<i32>().unwrap();
        let beta = neo.next().unwrap_or("") == "-beta";

        Self {
            minecraft,
            neoforge,
            beta,
        }
    }
}

impl<T: AsRef<str>> From<T> for NeoForgeVersion {
    fn from(value: T) -> Self {
        Self::new(value)
    }
}
