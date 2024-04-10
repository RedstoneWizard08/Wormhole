#[macro_export]
macro_rules! sources {
    ($enum: ident: $($var: ident = ($id: expr, $name: expr, $url: expr);)*) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub enum $enum {
            $($var,)*
        }

        impl $enum {
            pub fn source(&self) -> $crate::source::Source {
                match self.clone() {
                    $(Self::$var => $crate::source::Source {
                        id: Some($id),
                        name: $name.to_string(),
                        base_url: $url.to_string(),
                    },)*
                }
            }

            pub fn id(&self) -> i32 {
                self.source().id.unwrap_or(-1)
            }

            pub fn name(&self) -> String {
                self.source().name
            }

            pub fn base_url(&self) -> String {
                self.source().base_url
            }
        }

        #[allow(clippy::from_over_into)]
        impl Into<i32> for $enum {
            fn into(self) -> i32 {
                self.id()
            }
        }

        impl From<$enum> for $crate::source::Source {
            fn from(val: $enum) -> Self {
                val.source()
            }
        }
    }
}
