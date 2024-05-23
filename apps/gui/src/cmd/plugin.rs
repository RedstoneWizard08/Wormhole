use crate::{plugin_fn_proxy, AppState};
use api::plugin::PluginInfo;
use data::{instance::Instance, source::SourceMapping};
use query::{
    mod_::{Mod, ModVersion},
    source::{Paginated, QueryOptions},
};

plugin_fn_proxy!(async info => info: () -> [opt] PluginInfo);
plugin_fn_proxy!(async search_mods => search_mods: (resolver: SourceMapping, instance: Instance, query: Option<String>, opts: Option<QueryOptions>) -> [opt] Paginated<Mod>);
plugin_fn_proxy!(async get_mod => get_mod: (resolver: SourceMapping, mid: String) -> [opt] Mod);
plugin_fn_proxy!(async get_mod_versions => get_mod_versions: (resolver: SourceMapping, instance: Instance, mid: String) -> [opt] Vec<ModVersion>);
plugin_fn_proxy!(async get_mod_version => get_mod_version: (resolver: SourceMapping, instance: Instance, mid: String, version: String) -> [opt] ModVersion);
plugin_fn_proxy!(async get_latest_version => get_latest_version: (resolver: SourceMapping, instance: Instance, mid: String) -> [opt] ModVersion);
plugin_fn_proxy!(async get_download_url => get_download_url: (resolver: SourceMapping, instance: Instance, project: String, version: Option<String>) -> [opt] String);
plugin_fn_proxy!(async launch_game => launch_game: (instance: Instance) -> ());
plugin_fn_proxy!(async sources => sources: () -> [opt] Vec<String>);
