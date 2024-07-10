# rpc-rs

A blazingly fast, lightweight, and type-safe RPC/IPC framework for
Rust and Web/Tauri-based applications, based on `specta`.

## Introduction

`rpc-rs` is a type-safe Rust framework for building JSON RPCs and IPCs for use with Web
technologies, such as JavaScript and TypeScript. It follows the basic CRUD (Create, Read,
Update, Delete) pattern, and its syntax allows developers to easily create complex protocols
with relatively simple code.

## Examples

See [here](https://github.com/RedstoneWizard08/Wormhole/blob/main/crates/commands/src/router.rs)
for a great example of how to use this library.

## Integrations

rpc-rs integrates extremely well with the following libraries and tools:

- `axum`
- `tauri`
- `specta`
- `serde`
- `serde_json`
- `prisma-client-rust` (Custom build)
