#[macro_export]
macro_rules! fn_alias {
    ($var: ident::$real: ident => $name: ident: ($($arg: ident: $ty: ident),*)$( -> $ret: ident)?) => {
        #[tauri::command]
        #[specta::specta]
        fn $name(me: tauri::State<'_, $var>, $($arg: $ty),*) $(-> $ret)? {
            me.$real($($arg),*)
        }
    };

    (dyn $var: ident::$real: ident => $name: ident: ($($arg: ident: $ty: ident),*)$( -> $ret: ident)?) => {
        #[tauri::command]
        #[specta::specta]
        fn $name(me: tauri::State<'_, std::sync::Arc<Box<dyn $var + Send + Sync>>>, $($arg: $ty),*) $(-> $ret)? {
            me.$real($($arg),*)
        }
    };

    (dyn $var: ident::$real: ident => $name: ident: async ($($arg: ident: $ty: ident),*)$( -> $ret: ident)?) => {
        #[tauri::command]
        #[specta::specta]
        async fn $name<'a>(me: tauri::State<'a, std::sync::Arc<Box<dyn $var + Send + Sync + 'static>>>, $($arg: $ty),*) $(-> $ret)? {
            me.$real($($arg),*).await.ok_or(false)
        }
    };
}

mod _private {
    macro_rules! tauri_aliases {
        () => {
            // I don't feel like adding generic support to my macro, so here we are.

            type PluginInfoResult = Result<PluginInfo, bool>;
            type ModsResult = Result<Paginated<Mod>, bool>;
            type ModResult = Result<Mod, bool>;
            type VersionsResult = Result<Vec<ModVersion>, bool>;
            type VersionResult = Result<ModVersion, bool>;
            type StringResult = Result<String, bool>;

            type StringOpt = Option<String>;
            type OptionsOpt = Option<QueryOptions>;

            fn_alias!(
                dyn TauriPluginTrait::info => info:
                async () -> PluginInfoResult
            );

            fn_alias!(
                dyn TauriPluginTrait::search_mods => search_mods:
                async (resolver: SourceMapping, query: StringOpt, opts: OptionsOpt) -> ModsResult
            );

            fn_alias!(
                dyn TauriPluginTrait::get_mod => get_mod:
                async (resolver: SourceMapping, id: String) -> ModResult
            );

            fn_alias!(
                dyn TauriPluginTrait::get_mod_versions => get_mod_versions:
                async (resolver: SourceMapping, id: String) -> VersionsResult
            );

            fn_alias!(
                dyn TauriPluginTrait::get_mod_version => get_mod_version:
                async (resolver: SourceMapping, id: String, version: String) -> VersionResult
            );

            fn_alias!(
                dyn TauriPluginTrait::get_download_url => get_download_url:
                async (resolver: SourceMapping, project: String, version: StringOpt) -> StringResult
            );
        }
    }

    macro_rules! cmds {
        () => {
            (
                collect_functions![
                    info,
                    search_mods,
                    get_mod,
                    get_mod_versions,
                    get_mod_version,
                    get_download_url,
                ],
                Box::new(generate_handler![
                    info,
                    search_mods,
                    get_mod,
                    get_mod_versions,
                    get_mod_version,
                    get_download_url,
                ]),
            )
        };
    }

    pub(crate) use {cmds, tauri_aliases};
}

pub(crate) use _private::{cmds, tauri_aliases};
