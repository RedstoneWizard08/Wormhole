use std::path::PathBuf;

use crate::mod_::{Mod as RealMod, ModVersion};
use anyhow::Result;
use data::{conv::DbIntoArg, instance::Instance, mod_::DbMod, schema::mods, Conn};
use diesel::{insert_into, select, ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};

impl DbIntoArg<DbMod, (ModVersion, Instance, Vec<PathBuf>)> for RealMod {
    fn db_into_arg(
        self,
        db: &mut Conn,
        (version, instance, paths): (ModVersion, Instance, Vec<PathBuf>),
    ) -> Result<DbMod> {
        let path = serde_json::to_string(
            &paths
                .iter()
                .map(|v| {
                    v.strip_prefix(instance.data_dir())
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .to_string()
                })
                .collect::<Vec<_>>(),
        )?;

        let mut obj = DbMod {
            id: None,
            mod_id: self.id,
            version_id: Some(version.id.clone()),
            file_name: version.file_name(),
            hash: version.hash,
            size: version.size.map(|v| v.parse().unwrap()),
            name: self.name,
            source_id: Some(self.source),
            instance_id: instance.id,
            path,
        };

        let existing: Vec<DbMod> = mods::table
            .select(DbMod::as_select())
            .filter(mods::file_name.eq(&obj.file_name))
            .filter(mods::instance_id.eq(instance.id))
            .load(db)?;

        if let Some(it) = existing.get(0) {
            obj = it.clone();
        } else {
            obj = insert_into(mods::table)
                .values(&obj)
                .returning(DbMod::as_returning())
                .get_result(db)?;
        }

        Ok(obj)
    }
}
