//! Filtered macros for `rpc-rs`.

/// A macro to create a module for a prisma-client-rust module.
/// Your client must be an `Arc<...>`, as the state of the router.
/// This will work with a `Vec<...>` of the model.
///
/// Params:
/// - `client` - The path to the `PrismaClient` struct.
/// - `module` - The path to the module inside the generated `prisma` mod.
/// - `container` - The module's name. If you use `client.xxx.create(...)`, the container would be `xxx`.
/// - `primary_key` - The name of the model's primary key.
#[cfg(feature = "prisma")]
#[macro_export]
macro_rules! prisma_multi_module_filtered {
    {
        client: $client: ty,
        module: $module: ident,
        container: $cont: ident,
        primary_key: $pkey: ident,
        filter: $filter: ident = $filter_ty: ident,
    } => {
        $crate::module::Module::builder()
                .read($crate::proc::wrap(|cx: Arc<$client>, filt: $filter_ty| async move {
                    cx.$cont()
                        .find_many(vec![$module::$filter::equals(filt)])
                        .exec()
                        .await
                        .unwrap_or_default()
                }))
                .create($crate::proc::wrap(
                    |cx: Arc<$client>, m: Vec<$module::CreateUnchecked>| async move {
                        cx.$cont()
                            .create_many(m)
                            .exec()
                            .await
                            .map_err(|v| v.to_string())
                    },
                ))
                .delete($crate::proc::wrap(|cx: Arc<$client>, ids: Vec<i32>| async move {
                    cx.$cont()
                        .delete_many(ids.iter().map(|v| $module::$pkey::equals(v.clone())).collect())
                        .exec()
                        .await
                        .map_err(|v| v.to_string())
                }))
                .build()
    };
}

/// A macro to create a module for a prisma-client-rust module.
/// Your client must be an `Arc<...>`, as the state of the router.
///
/// Params:
/// - `router` - The variable containing router (must be `mut`).
/// - `single` - The name for the single route.
/// - `multi` - The name for the multi route.
/// - `client` - The path to the `PrismaClient` struct.
/// - `module` - The path to the module inside the generated `prisma` mod.
/// - `container` - The module's name. If you use `client.xxx.create(...)`, the container would be `xxx`.
/// - `primary_key` - The name of the model's primary key.
#[cfg(feature = "prisma")]
#[macro_export]
macro_rules! prisma_module_filtered {
    ($router: ident += [$single: expr, $multi: expr] {
        client: $client: ty,
        module: $module: ident,
        container: $cont: ident,
        primary_key: $pkey: ident,
        filter: $filter: ident = $filter_id: ident,
    }) => {
        $router = $router.mount(
            $single,
            $crate::prisma_single_module! {
                client: $client,
                module: $module,
                container: $cont,
                primary_key: $pkey,
            },
        );

        $router = $router.mount(
            $multi,
            $crate::prisma_multi_module_filtered! {
                client: $client,
                module: $module,
                container: $cont,
                primary_key: $pkey,
                filter: $filter = $filter_id,
            },
        );
    };
}
