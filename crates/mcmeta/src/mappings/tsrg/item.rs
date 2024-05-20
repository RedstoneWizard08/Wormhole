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
