//! A custom version of the [`specta::ts::export_function_header`] function.

use specta::{
    function::FunctionDataType,
    ts::{datatype, ExportConfig, Result},
    TypeMap,
};

/// Convert a [FunctionDataType] into a function header like would be used in a `.d.ts` file.
/// If your function requires a function body you can copy this function into your own codebase.
///
/// Eg. `function name()`
/// 
/// This had to be modified to support async `Promise`s and to not include the final semicolon.
pub fn export_function_header(dt: FunctionDataType, config: &ExportConfig) -> Result<String> {
    let type_map = TypeMap::default();
    let mut s = String::new();

    s.push_str("export ");

    if dt.asyncness {
        s.push_str("async ");
    }

    s.push_str("function ");
    s.push_str(&dt.name);
    s.push_str("(");

    for (i, (name, ty)) in dt.args.into_iter().enumerate() {
        if i != 0 {
            s.push_str(", ");
        }

        s.push_str(&name);
        s.push_str(": ");
        s.push_str(&datatype(config, &ty, &type_map)?);
    }

    s.push_str(")");

    if let Some(ty) = dt.result {
        s.push_str(": ");

        if dt.asyncness {
            s.push_str(&format!("Promise<{}>", datatype(config, &ty, &type_map)?));
        } else {
            s.push_str(&datatype(config, &ty, &type_map)?);
        }
    }

    Ok(s)
}
