use super::item::MappingItem;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tsrg2Mappings {
    pub names: Vec<String>,
    pub items: Vec<MappingItem>,
}

impl Tsrg2Mappings {
    pub fn serialize(&self) -> String {
        let mut buf = Vec::new();

        buf.push(format!("tsrg2 {}", self.names.join(" ")));

        for item in &self.items {
            item.serialize(&mut buf);
        }

        buf.join("\n")
    }
}
