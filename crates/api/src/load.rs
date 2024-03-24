use crate::{res::PluginResult, INIT_FN_NAME};
use anyhow::Result;
use wasi_experimental_http_wasmtime::{HttpCtx, HttpState};
use wasmtime::{Engine, Linker, Module, Store};
use wasmtime_wasi::{add_to_linker, WasiCtxBuilder};

pub fn load_wasi_plugin(path: &str) -> Result<()> {
    let engine = Engine::default();
    let module = Module::from_file(&engine, &path)?;
    let mut linker = Linker::new(&engine);

    add_to_linker(&mut linker, |s| s)?;

    HttpState::new()?.add_to_linker(&mut linker, |_| HttpCtx {
        allowed_hosts: Some(vec!["insecure:allow-all".to_string()]),
        max_concurrent_requests: Some(100),
    })?;

    let wasi = WasiCtxBuilder::new()
        .inherit_stdio()
        .inherit_args()?
        .build();

    let mut store = Store::new(&engine, wasi);

    linker.module(&mut store, "", &module)?;

    let res = PluginResult::from(
        linker
            .get(&mut store, "", INIT_FN_NAME)
            .unwrap()
            .into_func()
            .unwrap()
            .typed::<(), i32>(&store)?
            .call(&mut store, ())?,
    );

    println!("Result: {:?}", res);

    Ok(())
}
