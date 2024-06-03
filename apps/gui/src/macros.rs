/// Generate command/event handlers and functions.
#[macro_export]
macro_rules! commands {
    ($($map: ident),* $(,)?; $($command: path),* $(,)?; $($event: ident),* $(,)?) => {
        use $crate::commands::AppState;

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
