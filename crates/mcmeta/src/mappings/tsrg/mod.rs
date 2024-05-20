// MIT License
//
// Copyright (c) 2024 Jacob Sapoznikow <jacob1coder@gmail.com>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE. 

pub mod item;
pub mod tsrg;

use self::item::{MappingItem, MethodParam};
use super::util::{Consumable, IntoConsumable};
use crate::{
    eq_or_err,
    mappings::{tsrg::tsrg::Tsrg2Mappings, util::TrimExt},
};
use anyhow::Result;
use std::collections::HashMap;

pub fn parse_tsrg(lines: Vec<String>) -> Result<Tsrg2Mappings> {
    let mut iter = lines.iter().into_consumable();
    let mut header = iter.next()?.split(' ').into_consumable();
    let mut names = Vec::new();
    let mut items = Vec::new();

    eq_or_err!(header.next()?; == "tsrg2"; !=> "Invalid TSRG 2 header: {val}; Expected: {exp}");

    // This one is always the obfuscated one, and we can ignore that.
    header.next()?;

    while let Ok(item) = header.next() {
        names.push(item.to_string());
    }

    while let Ok(line) = iter.next() {
        let mut line = line.to_owned();
        let count = line.trim_start_count('\t');

        match count {
            0 => items.push(parse_class(line, &names)?),
            1 => items.push(parse_item(line, &mut iter, &names)?),

            _ => unreachable!(),
        };
    }

    Ok(Tsrg2Mappings { names, items })
}

pub fn parse_class(line: String, names: &Vec<String>) -> Result<MappingItem> {
    let mut iter = line.split(' ').into_consumable();
    let orig = iter.next()?.to_owned();
    let mut map = HashMap::new();

    for name in names {
        map.insert(name.to_owned(), iter.next()?.to_owned());
    }

    Ok(MappingItem::Class(orig, map))
}

pub fn parse_item(
    line: String,
    lines: &mut Consumable<&String>,
    names: &Vec<String>,
) -> Result<MappingItem> {
    let mut iter = line.split(' ').into_consumable();
    let orig = iter.next()?.to_owned();

    if !iter.peek()?.starts_with('(') {
        let mut map = HashMap::new();

        for name in names {
            map.insert(name.to_owned(), iter.next()?.to_owned());
        }

        Ok(MappingItem::Field(orig, map))
    } else {
        let desc = iter.next()?.to_owned();
        let mut map = HashMap::new();
        let mut is_static = false;
        let mut params = HashMap::new();

        for name in names {
            map.insert(name.to_owned(), iter.next()?.to_owned());
        }

        while lines.peek().map(|v| v.start_count('\t')).unwrap_or(0) == 2 {
            let line = lines.next()?.trim_start_matches('\t');

            if line.trim() == "static" {
                is_static = true;
            } else {
                let mut spl = line.split(' ').into_consumable();
                let idx = spl.next()?.parse::<usize>()?;
                let orig = spl.next()?.to_owned();
                let mut map = HashMap::new();

                for name in names {
                    map.insert(name.to_owned(), spl.next()?.to_owned());
                }

                let param = MethodParam {
                    name: orig,
                    names: map,
                };

                params.insert(idx, param);
            }
        }

        Ok(MappingItem::Method {
            name: orig,
            desc,
            names: map,
            is_static,
            params,
        })
    }
}
