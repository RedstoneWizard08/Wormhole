# rpc-rs

A blazingly fast, lightweight, idiomatic, and type-safe RPC/IPC framework for
Rust and Web/Tauri-based applications, based on `specta`.

## Introduction

`rpc-rs` is an idiomatic and type-safe Rust framework for building JSON RPCs and IPCs
for use with Web technologies, such as JavaScript and TypeScript. It follows the basic
CRUD (Create, Read, Update, Delete) pattern, and its syntax allows developers to easily
create complex applications with relatively simple code.

## Syntax

```rust
//! This code may not be completely accurate! I haven't yet built the library and this
//! is all plans! I also don't have ANY intellisense in a markdown file, so I can't check
//! for dumb syntactical errors!

use std::sync::Arc;
use serde::{Serialize, Deserialize};
use specta::Type;
use database::{prisma::{PrismaClient, game}, Game};
use rpc_rs::{Router, Module, ModuleBuilder, PrismaObject};

// The `PrismaObject` derive macro and trait handle the `db_params` function,
// which transforms this struct into a `Vec<SetParam>`. This struct is also
// read from the request body via Serde, hence the `Deserialize` derive macro
// being used here. The `#[prisma(module = ...)]` helper ensures that the derive
// macro finds the correct place to resolve all of the `SetParam` creation
// functions.
#[derive(Debug, Clone, Serialize, Deserialize, Type, PrismaObject)]
#[prisma(module = "database::prisma::game")]
pub struct GameCreation {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub curseforge: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thunderstore: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spacedock: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ckan: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modrinth: Option<bool>,
}

pub fn games_module(mut module: ModuleBuilder<State = Arc<PrismaClient>>) -> Module<State = Arc<PrismaClient>> {
    module
        // This closure is: FnMut(ModuleBuilder::State, GameCreation) -> impl Future<Output = Result<Game, Error>> + Send + Sync + 'static
        // The error conversion is handled automatically (via traits)
        .create(|db, data: GameCreation| (async move {
            // [C]RUD
            db.game().create(data.name.clone(), data.db_params()).exec().await
        })()) // <- Async closures have to be executed to get the `impl Future<...>`.
        // This closure is: FnMut(ModuleBuilder::State, i32) -> impl Future<Output = Result<Game, Error>> + Send + Sync + 'static
        .read(|db, id: i32| (async move { // The `id: i32` can be `_: ()` if you don't need any input here.
            // C[R]UD
            db.game().find_first(vec![game::id::equals(id)]).exec().await
        })())
        .update(...) // Insert your own functionality here.
        .delete(...)
        // All the operations get assigned to the `Option<...>` fields in the `ModuleBuilder`
        // struct. These will get reassigned to the ones in the `Module` struct and finalized
        // when the `build` method is called.
        .build()
}

pub fn build_router() -> Router<Arc<PrismaClient>> {
    Router::<Arc<PrismaClient>>::new()
        .attach(games_module) // Accepts: FnOnce(ModuleBuilder<State = Router::State>) -> Module<State = Router::State>
        // This closure is: FnMut(Router::State, ()) -> impl Future<Output = Result<Vec<Game>, Error>> + Send + Sync + 'static
        .query("games", |db, _: ()| (async move { // This is useful if you just need a read function.
            db.game().find_many(vec![]).exec().await
        })())
        // The `finish` function finalizes all the state holders and prepares it to be
        // mounted with either `axum` or `tauri`.
        .finish()
}

#[tokio::main]
pub async fn main() {
    let router = build_router();
    let db = Arc::new(PrismaClient::_builder().build().await.unwrap());
    // Creating the endpoint for axum or the plugin for tauri requires the state object.
    let endpoint = router.axum(db);

    let router = ...; // Axum stuff.

    router.nest("/rpc", endpoint);

    // Start your app with axum!
}
```

## Integrations

rpc-rs integrates extremely well with the following libraries and tools:

- `axum`
- `tauri`
- `specta`
- `serde`
- `serde_json`
