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
