//! Utility macros for Wormhole's commands.

/// Create a proxy function to a Tauri plugin.
#[macro_export]
macro_rules! plugin_fn_proxy {
    (async $name: ident => $fn: ident: () -> $ret: ty) => {
        #[allow(missing_docs)]
        #[$crate::whmacros::serde_call]
        #[$crate::tauri::command]
        #[$crate::specta::specta]
        pub async fn $name(game_id: i32, _pool: $crate::AppState<'_>) -> $crate::Result<$ret> {
            use $crate::whcore::errors::Stringify;

            let it = $crate::api::register::PLUGINS.lock().await;
            let plugin = it.get(&game_id).stringify()?;

            plugin.$fn().await.stringify()
        }
    };

    (async $name: ident => $fn: ident: () -> [opt] $ret: ty) => {
        #[allow(missing_docs)]
        #[$crate::whmacros::serde_call]
        #[$crate::tauri::command]
        #[$crate::specta::specta]
        pub async fn $name(game_id: i32, _pool: $crate::AppState<'_>) -> $crate::Result<$ret> {
            use $crate::whcore::errors::Stringify;

            let it = $crate::api::register::PLUGINS.lock().await;
            let plugin = it.get(&game_id).stringify()?;

            plugin.$fn().await.ok_or("Tried to unwrap a None value!".into())
        }
    };

    (async $name: ident => $fn: ident: ($($arg: ident: $arg_ty: ty),*) -> $ret: ty) => {
        #[allow(missing_docs)]
        #[whmacros::serde_call]
        #[tauri::command]
        #[specta::specta]
        pub async fn $name(game_id: i32, $($arg: $arg_ty),*, _pool: $crate::AppState<'_>) -> $crate::Result<$ret> {
            use $crate::whcore::errors::Stringify;

            let it = $crate::api::register::PLUGINS.lock().await;
            let plugin = it.get(&game_id).stringify()?;

            plugin.$fn($($arg),*).await.stringify()
        }
    };

    (async $name: ident => $fn: ident: ($($arg: ident: $arg_ty: ty),*) -> [opt] $ret: ty) => {
        #[allow(missing_docs)]
        #[$crate::whmacros::serde_call]
        #[$crate::tauri::command]
        #[$crate::specta::specta]
        pub async fn $name(game_id: i32, $($arg: $arg_ty),*, _pool: $crate::AppState<'_>) -> $crate::Result<$ret> {
            use $crate::whcore::errors::Stringify;

            let it = $crate::api::register::PLUGINS.lock().await;
            let plugin = it.get(&game_id).stringify()?;

            plugin.$fn($($arg),*).await.ok_or("Tried to unwrap a None value!".into())
        }
    };

    ($name: ident => $fn: ident: ($($arg: ident: $arg_ty: ty),*) -> $ret: ty) => {
        #[allow(missing_docs)]
        #[$crate::whmacros::serde_call]
        #[$crate::tauri::command]
        #[$crate::specta::specta]
        pub async fn $name(game_id: i32, $($arg: $arg_ty),*, _pool: $crate::AppState<'_>) -> $crate::Result<$ret> {
            use $crate::whcore::errors::Stringify;

            let it = $crate::api::register::PLUGINS.lock().await;
            let plugin = it.get(&game_id).stringify()?;

            plugin.$fn($($arg),*).ok_or("Tried to unwrap a None value!".into())
        }
    };

    ($name: ident => $fn: ident: () -> $ret: ty) => {
        #[$crate::whmacros::serde_call]
        #[$crate::tauri::command]
        #[$crate::specta::specta]
        pub async fn $name(game_id: i32, _pool: $crate::AppState<'_>) -> $crate::Result<$ret> {
            use $crate::whcore::errors::Stringify;

            let it = $crate::api::register::PLUGINS.lock().await;
            let plugin = it.get(&game_id).stringify()?;

            plugin.$fn().ok_or("Tried to unwrap a None value!".into())
        }
    };
}
