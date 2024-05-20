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
macro_rules! plugin_fn_proxy {
    (async $name: ident => $fn: ident: () -> $ret: ty) => {
        #[$crate::whmacros::serde_call]
        #[$crate::tauri::command]
        #[$crate::specta::specta]
        async fn $name(game_id: i32, _pool: $crate::AppState<'_>) -> Result<$ret, bool> {
            use $crate::whcore::Boolify;

            let it = $crate::api::register::PLUGINS.lock().unwrap();
            let plugin = it.get(&game_id).bool()?;

            $crate::futures::executor::block_on(plugin.$fn()).bool()
        }
    };

    (async $name: ident => $fn: ident: () -> [opt] $ret: ty) => {
        #[$crate::whmacros::serde_call]
        #[$crate::tauri::command]
        #[$crate::specta::specta]
        async fn $name(game_id: i32, _pool: $crate::AppState<'_>) -> Result<$ret, bool> {
            use $crate::whcore::Boolify;

            let it = $crate::api::register::PLUGINS.lock().unwrap();
            let plugin = it.get(&game_id).bool()?;

            $crate::futures::executor::block_on(plugin.$fn()).ok_or(false)
        }
    };
    
    (async $name: ident => $fn: ident: ($($arg: ident: $arg_ty: ty),*) -> $ret: ty) => {
        #[whmacros::serde_call]
        #[tauri::command]
        #[specta::specta]
        async fn $name(game_id: i32, $($arg: $arg_ty),*, _pool: $crate::AppState<'_>) -> Result<$ret, bool> {
            use $crate::whcore::Boolify;

            let it = $crate::api::register::PLUGINS.lock().unwrap();
            let plugin = it.get(&game_id).bool()?;

            $crate::futures::executor::block_on(plugin.$fn($($arg),*)).bool()
        }
    };

    (async $name: ident => $fn: ident: ($($arg: ident: $arg_ty: ty),*) -> [opt] $ret: ty) => {
        #[$crate::whmacros::serde_call]
        #[$crate::tauri::command]
        #[$crate::specta::specta]
        async fn $name(game_id: i32, $($arg: $arg_ty),*, _pool: $crate::AppState<'_>) -> Result<$ret, bool> {
            use $crate::whcore::Boolify;

            let it = $crate::api::register::PLUGINS.lock().unwrap();
            let plugin = it.get(&game_id).bool()?;

            $crate::futures::executor::block_on(plugin.$fn($($arg),*)).ok_or(false)
        }
    };

    ($name: ident => $fn: ident: ($($arg: ident: $arg_ty: ty),*) -> $ret: ty) => {
        #[$crate::whmacros::serde_call]
        #[$crate::tauri::command]
        #[$crate::specta::specta]
        async fn $name(game_id: i32, $($arg: $arg_ty),*, _pool: $crate::AppState<'_>) -> Result<$ret, bool> {
            use $crate::whcore::Boolify;

            let it = $crate::api::register::PLUGINS.lock().unwrap();
            let plugin = it.get(&game_id).bool()?;

            plugin.$fn($($arg),*).ok_or(false)
        }
    };

    ($name: ident => $fn: ident: () -> $ret: ty) => {
        #[$crate::whmacros::serde_call]
        #[$crate::tauri::command]
        #[$crate::specta::specta]
        async fn $name(game_id: i32, _pool: $crate::AppState<'_>) -> Result<$ret, bool> {
            use $crate::whcore::Boolify;

            let it = $crate::api::register::PLUGINS.lock().unwrap();
            let plugin = it.get(&game_id).bool()?;

            plugin.$fn().ok_or(false)
        }
    };
}
