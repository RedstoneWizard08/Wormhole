// MIT License
//
// Copyright (c) 2024 Jacob Sapoznikow <jacob1coder@gmail.com>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE. 

#[macro_export]
macro_rules! sources {
    ($enum: ident: $($var: ident = ($id: expr, $name: expr, $url: expr);)*) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Type)]
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

        impl From<$crate::source::Source> for $enum {
            fn from(val: $crate::source::Source) -> Self {
                match val.id.unwrap_or_default() {
                    $($id => $enum::$var,)*

                    _ => panic!("Unknown source!"),
                }
            }
        }

        #[repr(i32)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Type)]
        pub enum SourceMapping {
            $($var = $id,)*
        }

        impl SourceMapping {
            pub fn as_str(&self) -> &'static str {
                match self {
                    $(Self::$var => $name,)*
                }
            }
        }

        impl From<$enum> for SourceMapping {
            fn from(val: $enum) -> SourceMapping {
                match val {
                    $($enum::$var => SourceMapping::$var,)*
                }
            }
        }

        impl From<SourceMapping> for $enum {
            fn from(val: SourceMapping) -> $enum {
                match val {
                    $(SourceMapping::$var => $enum::$var,)*
                }
            }
        }
    }
}
