//! Utility macros for the plugins.

/// Automatically implement the [`UnityPlugin`] trait for a plugin.
#[macro_export]
macro_rules! unity_plugin {
    (
        $mod: ident::$name: ident = {
            id: $id: expr,
            name: $display: expr,
            game: $game: expr,
            exec: $exec: expr,
            dir: $dir: expr,
            icon: $icon: expr,
            banner: $banner: expr,
            
            resolvers: [
                $($resolver: ident),*
            ]
        }
    ) => {
        #[allow(non_snake_case)]
        mod $mod {
            const ICON_BYTES: &[u8] = include_bytes!($icon);
            const BANNER_BYTES: &[u8] = include_bytes!($banner);

            #[allow(missing_docs)]
            #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
            pub struct $name;

            #[$crate::async_trait::async_trait]
            impl $crate::api::unity::UnityPlugin for $name {
                fn new() -> Self
                where
                    Self: Sized,
                {
                    Self
                }

                fn id(&self) -> &'static str {
                    $id
                }

                fn game(&self) -> i32 {
                    $game
                }

                fn icon(&self) -> String {
                    format!("data:image/png;base64,{}", $crate::base64::Engine::encode(&$crate::base64::engine::general_purpose::STANDARD, ICON_BYTES))
                }

                fn banner(&self) -> String {
                    format!("data:image/png;base64,{}", $crate::base64::Engine::encode(&$crate::base64::engine::general_purpose::STANDARD, BANNER_BYTES))
                }

                fn display(&self) -> String {
                    $display.into()
                }

                fn fallback(&self) -> Option<&'static str> {
                    Some("BepInEx/plugins")
                }

                fn executable(&self) -> &'static str {
                    $exec
                }

                fn find(&self) -> Option<std::path::PathBuf> {
                    use $crate::whcore::finder::finder::InstallFinder;

                    $crate::whcore::finder::steam::Steam::new().find_game($dir).unwrap()
                }

                fn name(&self) -> &'static str {
                    $id
                }

                async fn resolvers(&self) -> Vec<Box<dyn $crate::query::source::Resolver + Send + Sync>> {
                    use $crate::query::source::Resolver;

                    vec![
                        $(Box::new($crate::query::$resolver::new().await)),*
                    ]
                }
            }
        }

        pub use $mod::$name;
    };
}