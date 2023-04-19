use crate::cli::ModCommands;
use ckandex::{Mod, ModType, QueryBuilder, IdFilter};
use wormhole_common::ckan::{query::get_mods, setup_ckan};

pub mod install;

use wormhole_common::ckan::{KSP1_CACHE_CLIENT, KSP2_CACHE_CLIENT};
use wormhole_common::instances::KSPGame;

pub async fn match_command(command: ModCommands) -> bool {
    println!("+ Setting up NetKAN repositories...");

    setup_ckan().await;

    match command {
        ModCommands::Browse {
            game_id,
            name_filter,
        } => {
            println!("+ Querying mods...");

            let mods = get_mods(game_id).await;

            println!("+ Filtering mods...");

            let all_mods = [mods.netkans, mods.frozen].concat();

            let mods: Vec<Mod> = all_mods
                .iter()
                .filter(|item| {
                    let mut ok = !item.name.eq("");

                    if ok {
                        if let Some(name_filter) = name_filter.clone() {
                            ok = item
                                .name
                                .to_lowercase()
                                .contains(&name_filter.to_lowercase());
                        }
                    }

                    return ok;
                })
                .cloned()
                .collect();

            println!("+ Found {} mods!", mods.len());

            for item in mods {
                let kind = match item.kind {
                    ModType::NETKAN => "Live",
                    ModType::FROZEN => "Frozen",
                };

                println!(
                    "=> {} ({}) [{}] | {}",
                    item.name,
                    item.id,
                    kind,
                    item.description.unwrap_or(" ".to_string())
                );
            }

            return true;
        }

        ModCommands::Info { id, game_id } => {
            let mut query = QueryBuilder::new();
            let query = query.add(IdFilter::new(id)).build();

            let mut client = KSP1_CACHE_CLIENT.lock().unwrap();

            if game_id == KSPGame::KSP2 {
                client = KSP2_CACHE_CLIENT.lock().unwrap();
            }

            let client = client.clone().unwrap();
            let resp = query.execute(client);

            println!("{:?}", resp);

            return true;
        }

        ModCommands::Install { id, instance_id, game_id } => {
            return false;
        }

        ModCommands::Remove { id, instance_id, game_id } => {
            return false;
        }
    };
}
