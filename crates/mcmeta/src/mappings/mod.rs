use std::collections::HashMap;

use ::proguard::{ProguardMapping, ProguardRecord};

use self::tsrg::{item::MappingItem, tsrg::Tsrg2Mappings};

pub mod macros;
pub mod proguard;
pub mod tsrg;
pub mod util;

pub fn merge_mappings(mut tsrg: Tsrg2Mappings, proguard: ProguardMapping) -> Tsrg2Mappings {
    let mut proguard_map = HashMap::new();

    for item in proguard.iter() {
        if let Ok(item) = item {
            if let ProguardRecord::Class {
                original,
                obfuscated,
            } = item
            {
                proguard_map.insert(obfuscated.to_owned(), original.to_owned());
            }
        }
    }

    let key = tsrg.names.first().unwrap().to_owned();
    tsrg.names = vec!["left".into(), "right".into()];

    for item in tsrg.items.iter_mut() {
        match item.clone() {
            MappingItem::Class(name, names) => {
                let mut map = HashMap::new();

                map.insert(
                    key.clone(),
                    proguard_map
                        .get(&name)
                        .unwrap_or(names.get(&key).unwrap())
                        .clone(),
                );

                *item = MappingItem::Class(name, map);
            }

            MappingItem::Field(name, names) => {
                let mut map = HashMap::new();

                map.insert(key.clone(), names.get(&key).unwrap().clone());

                *item = MappingItem::Field(name, map);
            }

            MappingItem::Method {
                name,
                desc,
                names,
                is_static,
                params,
            } => {
                let mut map = HashMap::new();

                map.insert(key.clone(), names.get(&key).unwrap().clone());

                *item = MappingItem::Method {
                    name,
                    desc,
                    names: map,
                    is_static,
                    params,
                };
            }
        }
    }

    tsrg
}
