//! Macros for the API.

/// Create a function alias to a member function
/// in a local variable.
#[macro_export]
macro_rules! fn_alias {
    ($var: ident::$real: ident => $name: ident: ($($arg: ident: $ty: ident),*)$( -> $ret: ident)?) => {
        #[allow(missing_docs)]
        fn $name(me: tauri::State<'_, $var>, $($arg: $ty),*) $(-> $ret)? {
            me.$real($($arg),*)
        }
    };

    (dyn $var: ident::$real: ident => $name: ident: ($($arg: ident: $ty: ident),*)$( -> $ret: ident)?) => {
        #[allow(missing_docs)]
        fn $name(me: tauri::State<'_, std::sync::Arc<Box<dyn $var + Send + Sync>>>, $($arg: $ty),*) $(-> $ret)? {
            me.$real($($arg),*)
        }
    };

    (dyn $var: ident::$real: ident => $name: ident: async ($($arg: ident: $ty: ident),*)$( -> $ret: ident)?) => {
        #[allow(missing_docs)]
        async fn $name<'a>(me: tauri::State<'a, std::sync::Arc<Box<dyn $var + Send + Sync + 'static>>>, $($arg: $ty),*) $(-> $ret)? {
            me.$real($($arg),*).await.ok_or(false)
        }
    };
}
