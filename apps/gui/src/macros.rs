#[macro_export]
macro_rules! plugin_fn_proxy {
    (async $name: ident => $fn: ident: () -> $ret: ty) => {
        #[$crate::whmacros::serde_call]
        #[$crate::tauri::command]
        #[$crate::specta::specta]
        pub async fn $name(game_id: i32, _pool: $crate::AppState<'_>) -> Result<$ret, bool> {
            use $crate::whcore::Boolify;

            let it = $crate::api::register::PLUGINS.lock().await;
            let plugin = it.get(&game_id).bool()?;

            plugin.$fn().await.bool()
        }
    };

    (async $name: ident => $fn: ident: () -> [opt] $ret: ty) => {
        #[$crate::whmacros::serde_call]
        #[$crate::tauri::command]
        #[$crate::specta::specta]
        pub async fn $name(game_id: i32, _pool: $crate::AppState<'_>) -> Result<$ret, bool> {
            use $crate::whcore::Boolify;

            let it = $crate::api::register::PLUGINS.lock().await;
            let plugin = it.get(&game_id).bool()?;

            plugin.$fn().await.ok_or(false)
        }
    };

    (async $name: ident => $fn: ident: ($($arg: ident: $arg_ty: ty),*) -> $ret: ty) => {
        #[whmacros::serde_call]
        #[tauri::command]
        #[specta::specta]
        pub async fn $name(game_id: i32, $($arg: $arg_ty),*, _pool: $crate::AppState<'_>) -> Result<$ret, bool> {
            use $crate::whcore::Boolify;

            let it = $crate::api::register::PLUGINS.lock().await;
            let plugin = it.get(&game_id).bool()?;

            plugin.$fn($($arg),*).await.bool()
        }
    };

    (async $name: ident => $fn: ident: ($($arg: ident: $arg_ty: ty),*) -> [opt] $ret: ty) => {
        #[$crate::whmacros::serde_call]
        #[$crate::tauri::command]
        #[$crate::specta::specta]
        pub async fn $name(game_id: i32, $($arg: $arg_ty),*, _pool: $crate::AppState<'_>) -> Result<$ret, bool> {
            use $crate::whcore::Boolify;

            let it = $crate::api::register::PLUGINS.lock().await;
            let plugin = it.get(&game_id).bool()?;

            plugin.$fn($($arg),*).await.ok_or(false)
        }
    };

    ($name: ident => $fn: ident: ($($arg: ident: $arg_ty: ty),*) -> $ret: ty) => {
        #[$crate::whmacros::serde_call]
        #[$crate::tauri::command]
        #[$crate::specta::specta]
        pub async fn $name(game_id: i32, $($arg: $arg_ty),*, _pool: $crate::AppState<'_>) -> Result<$ret, bool> {
            use $crate::whcore::Boolify;

            let it = $crate::api::register::PLUGINS.lock().await;
            let plugin = it.get(&game_id).bool()?;

            plugin.$fn($($arg),*).ok_or(false)
        }
    };

    ($name: ident => $fn: ident: () -> $ret: ty) => {
        #[$crate::whmacros::serde_call]
        #[$crate::tauri::command]
        #[$crate::specta::specta]
        pub async fn $name(game_id: i32, _pool: $crate::AppState<'_>) -> Result<$ret, bool> {
            use $crate::whcore::Boolify;

            let it = $crate::api::register::PLUGINS.lock().await;
            let plugin = it.get(&game_id).bool()?;

            plugin.$fn().ok_or(false)
        }
    };
}
