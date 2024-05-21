#[macro_export]
macro_rules! sources {
    ($ty: ident => $enum: ident = $mapping: ident: $($var: ident = ($id: expr, $name: expr, $url: expr);)*) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Type)]
        pub enum $enum {
            $($var,)*
        }

        impl $enum {
            pub fn source(&self) -> $ty {
                match self.clone() {
                    $(Self::$var => $ty {
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

        impl Into<$ty> for $enum {
            fn into(self) -> $ty {
                self.source()
            }
        }

        impl From<$ty> for $enum {
            fn from(val: $ty) -> Self {
                match val.id.unwrap_or_default() {
                    $($id => $enum::$var,)*

                    _ => panic!("Unknown source!"),
                }
            }
        }

        unsafe impl Send for $enum {}
        unsafe impl Sync for $enum {}

        #[repr(i32)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Type)]
        pub enum $mapping {
            $($var = $id,)*
        }

        impl $mapping {
            pub fn as_str(&self) -> &'static str {
                match self {
                    $(Self::$var => $name,)*
                }
            }
        }

        impl From<$enum> for $mapping {
            fn from(val: $enum) -> $mapping {
                match val {
                    $($enum::$var => $mapping::$var,)*
                }
            }
        }

        impl From<$mapping> for $enum {
            fn from(val: $mapping) -> $enum {
                match val {
                    $($mapping::$var => $enum::$var,)*
                }
            }
        }

        impl From<i32> for $mapping {
            fn from(val: i32) -> $mapping {
                match val {
                    $($id => $mapping::$var,)*

                    _ => panic!("Unsupported value for {}: {}", stringify!($mapping), val),
                }
            }
        }

        unsafe impl Send for $mapping {}
        unsafe impl Sync for $mapping {}
    }
}
