use crate::{PluginResult, INIT_FN_NAME};
use anyhow::Result;
use wasi_common::sync::{add_to_linker, WasiCtxBuilder};
use wasmtime::{Engine, Linker, Module, Store};

pub fn load_wasi_plugin(path: &str) -> Result<()> {
    let engine = Engine::default();
    let mut linker = Linker::new(&engine);

    add_to_linker(&mut linker, |s| s)?;

    let wasi = WasiCtxBuilder::new()
        .inherit_stdio()
        .inherit_args()?
        .build();

    let mut store = Store::new(&engine, wasi);
    let module = Module::from_file(&engine, &path)?;

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
