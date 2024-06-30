#[macro_export]
macro_rules! sources_enum {
    ($($source: ident),*) => {
        pub enum Sources {
            $($source),*
        }

        impl Sources {
            pub(crate) fn create_source(&self) -> $crate::Source {
                match self {
                    $(
                        Self::$source => $crate::Source {
                            games: None,
                            id: -1,
                            mods: None,
                            name: String::from(stringify!($source)),
                        }
                    ),*
                }
            }

            pub fn values() -> Vec<Sources> {
                vec![
                    $(Self::$source),*
                ]
            }

            pub async fn source(&self, client: std::sync::Arc<crate::prisma::PrismaClient>) -> Result<$crate::Source> {
                client
                    .source()
                    .find_first(vec![$crate::prisma::source::name::equals(self.create_source().name)])
                    .exec()
                    .await?
                    .ok_or(anyhow!("Could not find that source in the database!"))
            }

            pub async fn source_alt(&self) -> Result<$crate::Source> {
                $crate::client().await?
                    .source()
                    .find_first(vec![$crate::prisma::source::name::equals(self.create_source().name)])
                    .exec()
                    .await?
                    .ok_or(anyhow!("Could not find that source in the database!"))
            }

            pub fn as_str(&self) -> &'static str {
                match self {
                    $(Self::$source => stringify!($source)),*
                }
            }
        }
    }
}
