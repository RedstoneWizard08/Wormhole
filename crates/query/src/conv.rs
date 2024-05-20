// MIT License
//
// Copyright (c) 2024 Jacob Sapoznikow <jacob1coder@gmail.com>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE. 

use crate::mod_::{Mod as RealMod, ModVersion};
use anyhow::Result;
use data::{conv::DbIntoArg, instance::Instance, mod_::Mod, schema::mods, Conn};
use diesel::{insert_into, RunQueryDsl, SelectableHelper};

impl DbIntoArg<Mod, (ModVersion, Instance)> for RealMod {
    fn db_into_arg(
        self,
        db: &mut Conn,
        (version, instance): (ModVersion, Instance),
    ) -> Result<Mod> {
        let obj = Mod {
            id: None,
            mod_id: self.id,
            version_id: Some(version.id.clone()),
            file_name: version.file_name(),
            hash: version.hash,
            size: version.size.map(|v| v.parse().unwrap()),
            name: self.name,
            source_id: Some(self.source),
            instance_id: instance.id,
        };

        let obj = insert_into(mods::table)
            .values(&obj)
            .returning(Mod::as_returning())
            .get_result(db)?;

        Ok(obj)
    }
}
