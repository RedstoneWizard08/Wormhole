//! Utility macros for Wormhole's commands.

/// Create a proxy function to a Tauri plugin.
#[macro_export]
macro_rules! plugin_fn_proxy {
    (async $name: ident => $fn: ident: () -> $ret: ty) => {
        #[allow(missing_docs)]
        pub async fn $name(game_id: i32) -> anyhow::Result<$ret> {
            let it = $crate::api::register::PLUGINS.lock().await;
            let plugin = it.get(&game_id)?;

            plugin.$fn().await
        }
    };

    (async $name: ident => $fn: ident: () -> [opt] $ret: ty) => {
        #[allow(missing_docs)]
        pub async fn $name(game_id: i32) -> anyhow::Result<$ret> {
            let it = $crate::api::register::PLUGINS.lock().await;
            let plugin = it.get(&game_id).ok_or("Tried to unwrap a None value!".into())?;

            plugin.$fn().await.ok_or("Tried to unwrap a None value!".into())
        }
    };

    (async $name: ident => $fn: ident: ($($arg: ident: $arg_ty: ty),*) -> $ret: ty) => {
        #[allow(missing_docs)]
        pub async fn $name(game_id: i32, $($arg: $arg_ty),*) -> anyhow::Result<$ret> {
            let it = $crate::api::register::PLUGINS.lock().await;
            let plugin = it.get(&game_id).ok_or("Tried to unwrap a None value!".into())?;

            plugin.$fn($($arg),*).await
        }
    };

    (async $name: ident => $fn: ident: ($($arg: ident: $arg_ty: ty),*) -> [opt] $ret: ty) => {
        #[allow(missing_docs)]
        pub async fn $name(game_id: i32, $($arg: $arg_ty),*) -> anyhow::Result<$ret> {
            let it = $crate::api::register::PLUGINS.lock().await;
            let plugin = it.get(&game_id).ok_or("Tried to unwrap a None value!".into())?;

            plugin.$fn($($arg),*).await.ok_or("Tried to unwrap a None value!".into())
        }
    };

    ($name: ident => $fn: ident: ($($arg: ident: $arg_ty: ty),*) -> $ret: ty) => {
        #[allow(missing_docs)]
        pub async fn $name(game_id: i32, $($arg: $arg_ty),*) -> anyhow::Result<$ret> {
            let it = $crate::api::register::PLUGINS.lock().await;
            let plugin = it.get(&game_id)?;

            plugin.$fn($($arg),*).ok_or("Tried to unwrap a None value!".into())
        }
    };

    ($name: ident => $fn: ident: () -> $ret: ty) => {
        #[allow(missing_docs)]
        pub async fn $name(game_id: i32) -> anyhow::Result<$ret> {
            let it = $crate::api::register::PLUGINS.lock().await;
            let plugin = it.get(&game_id)?;

            plugin.$fn().ok_or("Tried to unwrap a None value!".into())
        }
    };
}

/// Apply a methods to an [`rspc::Router`].
#[macro_export]
macro_rules! apply {
    ($r: ident => $p: path) => {
        let $r = $p($r);
    };

    ($r: ident: $($p: path),*) => {
        $(let $r = $p($r);)*
    };
}
