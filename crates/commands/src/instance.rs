//! Instance commands for the GUI.

use std::sync::Arc;

use anyhow::Result;
use api::register::PLUGINS;
use data::{
    prisma::{
        game,
        instance::{self, SetParam},
        PrismaClient,
    },
    Instance,
};
use mcmeta::cmd::modded::GetLoader;
use whcore::traits::Resultify;

/// Gets all instances for the given game.
///
/// Arguments:
/// - `game_id` - The game's ID in the database.
///
/// See: [`Instance`]
pub async fn get_instances(game_id: i32, db: Arc<PrismaClient>) -> Result<Vec<Instance>> {
    Ok(db
        .instance()
        .find_many(vec![instance::game_id::equals(game_id)])
        .exec()
        .await?)
}

/// Deletes an instance.
///
/// Arguments:
/// - `instance_id` - The instance's ID in the database.
pub async fn delete_instance(instance_id: i32, db: Arc<PrismaClient>) -> Result<()> {
    db.instance()
        .delete(instance::id::equals(instance_id))
        .exec()
        .await?;

    Ok(())
}

/// Creates a new instance.
/// This will also install whatever its default mod loader is.
///
/// Example: For Minecraft, this will install whatever the latest version
///          of the vanilla game is.
///
/// Arguments:
/// - `name` - The instance's name.
/// - `game_id` - The game's ID in the database.
pub async fn create_instance(
    name: String,
    game_id: i32,
    db: Arc<PrismaClient>,
) -> Result<Instance> {
    let lock = PLUGINS.lock().await;
    let plugin = lock.get(&game_id).resultify()?;
    let dirs = plugin.dirs();

    info!("Creating instance...");

    let instance = instance::create(
        name.clone(),
        game::id::equals(game_id),
        dirs.data
            .join("instances")
            .join(&name)
            .to_str()
            .unwrap()
            .to_string(),
        dirs.cache.to_str().unwrap().to_string(),
        plugin.find().resultify()?.to_str().unwrap().to_string(),
        vec![],
    );

    info!("Installing loader...");

    let mut setters: Vec<SetParam> = Vec::new();

    setters.push(instance::loader::set(Some(serde_json::to_string(
        &instance.loader().await?,
    )?)));

    info!("Updating database...");

    let it = db
        .instance()
        .create(
            name.clone(),
            game::id::equals(game_id),
            dirs.data
                .join("instances")
                .join(&name)
                .to_str()
                .unwrap()
                .to_string(),
            dirs.cache.to_str().unwrap().to_string(),
            plugin.find().resultify()?.to_str().unwrap().to_string(),
            setters,
        )
        .exec()
        .await?;

    info!("Installing instance...");

    plugin.install_instance(&it).await?;

    Ok(it)
}

/// Updates an instance's description.
///
/// Arguments:
/// - `instance_id` - The instance's ID in the database.
/// - `desc` - The new description.
// Realistically, we won't be updating anything but the description for now.
// If this changes in the future, I'll expand this.
pub async fn update_instance(
    instance_id: i32,
    desc: String,
    db: Arc<PrismaClient>,
) -> Result<Instance> {
    Ok(db
        .instance()
        .update(
            instance::id::equals(instance_id),
            vec![instance::description::set(desc)],
        )
        .exec()
        .await?)
}

/// Gets an instance by its ID.
///
/// Arguments:
/// - `instance_id` - The instance's ID in the database.
pub async fn get_instance(instance_id: i32, db: Arc<PrismaClient>) -> Result<Instance> {
    db.instance()
        .find_first(vec![instance::id::equals(instance_id)])
        .exec()
        .await?
        .resultify()
}
