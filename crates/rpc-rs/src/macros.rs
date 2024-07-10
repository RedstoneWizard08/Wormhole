//! Macros for `rpc-rs`.

/// A macro to create a module for a prisma-client-rust module.
/// Your client must be an `Arc<...>`, as the state of the router.
///
/// Params:
/// - `client` - The path to the `PrismaClient` struct.
/// - `module` - The path to the module inside the generated `prisma` mod.
/// - `container` - The module's name. If you use `client.xxx.create(...)`, the container would be `xxx`.
/// - `primary_key` - The name of the model's primary key.
#[macro_export]
macro_rules! prisma_single_module {
    {
        client: $client: ty,
        module: $module: ident,
        container: $cont: ident,
        primary_key: $pkey: ident,
    } => {
        Module::builder()
                .read(wrap(|cx: Arc<$client>, item_id: i32| async move {
                    cx.$cont()
                        .find_first(vec![$module::$pkey::equals(item_id)])
                        .exec()
                        .await
                        .unwrap_or_default()
                }))
                .create(wrap(
                    |cx: Arc<$client>, item: $module::CreateUnchecked| async move {
                        cx.$cont()
                            .create_many(vec![item])
                            .exec()
                            .await
                            .map_err(|v| v.to_string())
                    },
                ))
                .update(wrap(
                    |cx: Arc<$client>, update: $module::Update| async move {
                        cx.$cont()
                            .update($module::$pkey::equals(update.id), update.as_params())
                            .exec()
                            .await
                            .map_err(|v| v.to_string())
                    },
                ))
                .delete(wrap(
                    |cx: Arc<$client>, item_id: i32| async move {
                        cx.$cont()
                            .delete($module::$pkey::equals(item_id))
                            .exec()
                            .await
                            .map_err(|v| v.to_string())
                    },
                ))
                .build()
    };
}

/// A macro to create a module for a prisma-client-rust module.
/// Your client must be an `Arc<...>`, as the state of the router.
/// This will work with a `Vec<...>` of the model.
///
/// Params:
/// - `client` - The path to the `PrismaClient` struct.
/// - `module` - The path to the module inside the generated `prisma` mod.
/// - `container` - The module's name. If you use `client.xxx.create(...)`, the container would be `xxx`.
/// - `primary_key` - The name of the model's primary key.
#[macro_export]
macro_rules! prisma_multi_module {
    {
        client: $client: ty,
        module: $module: ident,
        container: $cont: ident,
        primary_key: $pkey: ident,
    } => {
        Module::builder()
                .read(wrap(|cx: Arc<$client>, _: ()| async move {
                    cx.$cont()
                        .find_many(vec![])
                        .exec()
                        .await
                        .unwrap_or_default()
                }))
                .create(wrap(
                    |cx: Arc<$client>, m: Vec<$module::CreateUnchecked>| async move {
                        cx.$cont()
                            .create_many(m)
                            .exec()
                            .await
                            .map_err(|v| v.to_string())
                    },
                ))
                .delete(wrap(|cx: Arc<$client>, ids: Vec<i32>| async move {
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
#[macro_export]
macro_rules! prisma_module {
    ($router: ident += [$single: expr, $multi: expr] {
        client: $client: ty,
        module: $module: ident,
        container: $cont: ident,
        primary_key: $pkey: ident,
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
            $crate::prisma_multi_module! {
                client: $client,
                module: $module,
                container: $cont,
                primary_key: $pkey,
            },
        );
    };
}
