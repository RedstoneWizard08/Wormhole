use crate::mod_::{Mod as RealMod, ModVersion};
use anyhow::Result;
use data::{conv::DbIntoArg, instance::Instance, mod_::DbMod, schema::mods, Conn};
use diesel::{insert_into, select, ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};

impl DbIntoArg<DbMod, (ModVersion, Instance)> for RealMod {
    fn db_into_arg(
        self,
        db: &mut Conn,
        (version, instance): (ModVersion, Instance),
    ) -> Result<DbMod> {
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
