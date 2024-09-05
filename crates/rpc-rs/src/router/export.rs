//! The [`specta`] export module.

use std::{collections::HashMap, fs, path::PathBuf, sync::Arc};

use specta::ts::{export_named_datatype, Result};

use super::{func::export_function_header, router::Router, Method};
use crate::{proc::GenericProcedure, util::TripleS};

pub(crate) const MODULE_STUB: &str = include_str!("./module_stub.ts");
pub(crate) const CORE: &str = include_str!("./core.ts");

impl<Cx: TripleS + Clone> Router<Cx> {
    /// Export a function's TypeScript version with [`specta`].
    pub(crate) fn export_func(
        &mut self,
        script: &mut Vec<String>,
        route_prefix: &String,
        func_name: &String,
        module_name: &String,
        method: Method,
        func: Arc<Box<dyn GenericProcedure<Cx> + TripleS>>,
    ) -> Result<()> {
        let func_ty = func.specta_type(func_name.clone().into(), &mut self.type_map);
        let arg = func_ty.args.first().unwrap().0.clone();
        let data = export_function_header(func_ty, &self.export_cfg)?;

        let data = format!(
            "{} {{\nreturn await __rpc_call(\"{}\", \"{}\", \"{}\", {});\n}}",
            data,
            route_prefix,
            method.as_str(),
            module_name,
            arg
        );

        script.push(data);

        Ok(())
    }

    /// Export an invoke function's TypeScript version with [`specta`].
    pub(crate) fn export_invoke_func(
        &mut self,
        script: &mut Vec<String>,
        route_prefix: &String,
        func_name: &String,
        invoke_name: &String,
        func: Arc<Box<dyn GenericProcedure<Cx> + TripleS>>,
    ) -> Result<()> {
        let func_ty = func.specta_type(func_name.clone().into(), &mut self.type_map);
        let arg = func_ty.args.first().unwrap().0.clone();
        let data = export_function_header(func_ty, &self.export_cfg)?;

        let data = format!(
            "{} {{\nreturn await __rpc_invoke(\"{}\", \"{}\", {});\n}}",
            data, route_prefix, invoke_name, arg
        );

        script.push(data);

        Ok(())
    }

    /// Export the entire router's bindings.
    /// Warning: This **does** consume the router!
    pub fn export(mut self, route_prefix: impl AsRef<str>, path: impl Into<PathBuf>) -> Result<()> {
        let path: PathBuf = path.into();
        let route_prefix = route_prefix.as_ref().to_string();
        let mut code = Vec::new();
        let mut group_code = Vec::new();

        code.push(CORE.into());

        for (name, module) in self.modules.clone() {
            let funcs: HashMap<Method, Arc<Box<dyn GenericProcedure<Cx> + TripleS>>> =
                HashMap::from_iter([
                    (Method::Create, module.create),
                    (Method::Read, module.read),
                    (Method::Update, module.update),
                    (Method::Delete, module.delete),
                ]);

            for (method, func) in funcs {
                let new_name = format!("__rpc_call_{}_{}", &name, method.as_str());

                self.export_func(&mut code, &route_prefix, &new_name, &name, method, func)?;
            }

            let group = MODULE_STUB.replace("$name$", &name);

            code.push(group);
            group_code.push(format!("{}: __rpc_module_{},", &name, name));
        }

        let mut invoke_group_code = Vec::new();

        for (name, invoker) in self.invokers.clone() {
            let new_name = format!("__rpc_invoke_{}", &name);

            self.export_invoke_func(&mut code, &route_prefix, &new_name, &name, invoker)?;

            invoke_group_code.push(format!("{}: {},", name, new_name));
        }

        let invoke_group = format!(
            "const __rpc_invokers = {{\n{}\n}};",
            invoke_group_code.join("\n")
        );

        code.push(invoke_group);
        group_code.push("invoke: __rpc_invokers,".into());

        for (_, ty) in self.type_map.clone().iter() {
            code.push(export_named_datatype(
                &self.export_cfg,
                ty,
                &mut self.type_map,
            )?);
        }

        code.push(format!(
            "export const RPC = {{\n{}\n}};\n\nexport default RPC;",
            group_code.join("\n")
        ));

        let code = code.join("\n\n");

        if let Some(parent) = path.clone().parent() {
            if !parent.exists() {
                fs::create_dir_all(parent)?;
            }
        }

        fs::write(path, code)?;

        Ok(())
    }
}
