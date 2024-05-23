use std::{fs, path::PathBuf};

use anyhow::Result;
use data::{
    diesel::{delete, ExpressionMethods, RunQueryDsl},
    instance::Instance,
    mod_::DbMod,
    schema::mods,
    Conn,
};

pub async fn uninstall_mod(db: &mut Conn, item: DbMod, instance: Instance) -> Result<()> {
    let paths = serde_json::from_str::<Vec<PathBuf>>(&item.path)?;

    for item in paths {
        let path = instance.data_dir().join(item);

        if path.is_dir() {
            fs::remove_dir_all(path)?;
        } else {
            fs::remove_file(path)?;
        }
    }

    delete(mods::table)
        .filter(mods::id.eq(item.id))
        .execute(db)?;

    Ok(())
}
