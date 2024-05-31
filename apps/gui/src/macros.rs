/// Create a proxy function to a Tauri plugin.
#[macro_export]
macro_rules! plugin_fn_proxy {
    (async $name: ident => $fn: ident: () -> $ret: ty) => {
        #[allow(missing_docs)]
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
        #[allow(missing_docs)]
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
        #[allow(missing_docs)]
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
        #[allow(missing_docs)]
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
        #[allow(missing_docs)]
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

/// Generate command/event handlers and functions.
#[macro_export]
macro_rules! commands {
    ($($map: ident),* $(,)?; $($command: path),* $(,)?; $($event: ident),* $(,)?) => {
        $crate::whmacros::serde_funcs![$($command),*];

        /// Get the app's [`$crate::specta::functions::CollectFunctionsResult`].
        pub fn funcs() -> $crate::specta::functions::CollectFunctionsResult {
            let map = $crate::whcore::merge_type_maps(vec![$($map::type_map()),*]);

            $crate::specta::functions::collect_functions![map; $($command),*]
        }

        /// Get the app's command invoker.
        pub fn invoker<R: $crate::tauri::Runtime>() -> Box<dyn Fn($crate::tauri::Invoke<R>) + Send + Sync + 'static> {
            Box::new($crate::tauri::generate_handler![$($command),*])
        }

        /// Get the app's command invoker and [`$crate::specta::functions::CollectFunctionsResult`].
        pub fn cmds<R: $crate::tauri::Runtime>() -> (
            $crate::specta::functions::CollectFunctionsResult,
            Box<dyn Fn($crate::tauri::Invoke<R>) + Send + Sync + 'static>,
        ) {
            (funcs(), invoker())
        }

        /// Get [`tauri_specta`]'s events data.
        pub fn events<R: $crate::tauri::Runtime>() -> ($crate::tauri_specta::EventCollection, Vec<$crate::tauri_specta::EventDataType>, $crate::specta::TypeMap) {
            $crate::tauri_specta::collect_events![$($event),*]
        }

        /// Get the Tauri [`$crate::tauri::Context`] for Wormhole.
        pub fn ctx() -> $crate::tauri::Context<$crate::tauri::utils::assets::EmbeddedAssets> {
            $crate::tauri::generate_context!()
        }
    }
}
