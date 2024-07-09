use std::{collections::HashMap, fs, path::PathBuf, sync::Arc};

use specta::ts::{export_named_datatype, Result};

use super::{func::export_function_header, router::Router, Method};
use crate::{proc::GenericProcedure, util::TripleS};

pub(crate) const MODULE_STUB: &str = include_str!("./module_stub.ts");
pub(crate) const CORE: &str = include_str!("./core.ts");

impl<Cx: TripleS + Clone> Router<Cx> {
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

    pub fn export(mut self, route_prefix: impl AsRef<str>, path: impl Into<PathBuf>) -> Result<()> {
        let path = path.into();
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

        fs::write(path, code)?;

        Ok(())
    }
}
