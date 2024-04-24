use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MappingItem {
    /// A class item.
    /// This contains its original name and
    /// a list of its names for each mapping key.
    Class(String, HashMap<String, String>),

    /// A field item.
    /// This contains its original name and
    /// a list of its names for each mapping key.
    Field(String, HashMap<String, String>),

    /// A method item.
    /// This contains its original name, desc, and
    /// a list of its names for each mapping key.
    Method {
        name: String,
        desc: String,
        names: HashMap<String, String>,
        is_static: bool,
        params: HashMap<usize, MethodParam>,
    },
}

impl MappingItem {
    pub fn serialize(&self, buf: &mut Vec<String>) {
        match self.clone() {
            Self::Class(name, names) => {
                buf.push(format!(
                    "{} {}",
                    name,
                    names.values().cloned().collect::<Vec<_>>().join(" ")
                ));
            }

            Self::Field(name, names) => {
                buf.push(format!(
                    "\t{} {}",
                    name,
                    names.values().cloned().collect::<Vec<_>>().join(" ")
                ));
            }

            Self::Method {
                name,
                desc,
                names,
                is_static,
                params,
            } => {
                buf.push(format!(
                    "\t{} {} {}",
                    name,
                    desc,
                    names.values().cloned().collect::<Vec<_>>().join(" ")
                ));

                if is_static {
                    buf.push("\t\tstatic".into());
                }

                for (idx, param) in params {
                    buf.push(param.serialize(idx));
                }
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MethodParam {
    pub name: String,
    pub names: HashMap<String, String>,
}

impl MethodParam {
    pub fn serialize(&self, idx: usize) -> String {
        format!(
            "\t\t{} {} {}",
            idx,
            self.name,
            self.names.values().cloned().collect::<Vec<_>>().join(" ")
        )
    }
}
