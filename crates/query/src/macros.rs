#[macro_export]
macro_rules! stub_source {
    ($struct: ident) => {
        #[async_trait]
        #[cfg(target_arch = "wasm32")]
        impl Source for $struct {
            fn new() -> Self {
                unimplemented!()
            }

            fn new_with_token(_: String) -> Self {
                unimplemented!()
            }

            fn base(&self) -> String {
                unimplemented!()
            }

            async fn search(
                &self,
                _: i32,
                _: String,
                _: Option<$crate::source::QueryOptions>,
            ) -> $crate::anyhow::Result<$crate::source::Paginated<$crate::mod_::Mod>> {
                unimplemented!()
            }

            async fn get_mod(&self, _: String) -> $crate::anyhow::Result<$crate::mod_::Mod> {
                unimplemented!()
            }

            async fn get_versions(
                &self,
                _: String,
            ) -> $crate::anyhow::Result<Vec<$crate::mod_::ModVersion>> {
                unimplemented!()
            }

            async fn get_version(
                &self,
                _: String,
                _: String,
            ) -> $crate::anyhow::Result<$crate::mod_::ModVersion> {
                unimplemented!()
            }

            async fn get_download_url(
                &self,
                _: String,
                _: Option<String>,
            ) -> $crate::anyhow::Result<String> {
                unimplemented!()
            }
        }
    };
}
