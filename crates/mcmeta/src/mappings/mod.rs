// MIT License
//
// Copyright (c) 2024 Jacob Sapoznikow <jacob1coder@gmail.com>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE. 

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
