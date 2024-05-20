// MIT License
//
// Copyright (c) 2024 Jacob Sapoznikow <jacob1coder@gmail.com>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE. 

// Currently we cannot use WASI plugins, so this is useless.
// #[macro_export]
// macro_rules! stub_source {
//     ($struct: ident) => {
//         #[async_trait]
//         #[cfg(target_arch = "wasm32")]
//         impl Source for $struct {
//             fn new() -> Self {
//                 unimplemented!()
//             }

//             fn new_with_token(_: String) -> Self {
//                 unimplemented!()
//             }

//             fn base(&self) -> String {
//                 unimplemented!()
//             }

//             async fn search(
//                 &self,
//                 _: i32,
//                 _: String,
//                 _: Option<$crate::source::QueryOptions>,
//             ) -> $crate::anyhow::Result<$crate::source::Paginated<$crate::mod_::Mod>> {
//                 unimplemented!()
//             }

//             async fn get_mod(&self, _: String) -> $crate::anyhow::Result<$crate::mod_::Mod> {
//                 unimplemented!()
//             }

//             async fn get_versions(
//                 &self,
//                 _: String,
//             ) -> $crate::anyhow::Result<Vec<$crate::mod_::ModVersion>> {
//                 unimplemented!()
//             }

//             async fn get_version(
//                 &self,
//                 _: String,
//                 _: String,
//             ) -> $crate::anyhow::Result<$crate::mod_::ModVersion> {
//                 unimplemented!()
//             }

//             async fn get_download_url(
//                 &self,
//                 _: String,
//                 _: Option<String>,
//             ) -> $crate::anyhow::Result<String> {
//                 unimplemented!()
//             }
//         }
//     };
// }
