//! A custom version of the [`specta::ts::export_function_header`] function.

use specta::{
    function::FunctionDataType,
    ts::{ExportConfig, Result},
    DataType, LiteralType, TypeMap,
};

/// A workaround for [`LiteralType::to_ts`] being private.
pub trait ToTs {
    /// Get the TypeScript type name.
    fn to_ts(&self) -> String;
}

impl ToTs for LiteralType {
    fn to_ts(&self) -> String {
        match self.clone() {
            Self::i8(v) => v.to_string(),
            Self::i16(v) => v.to_string(),
            Self::i32(v) => v.to_string(),
            Self::u8(v) => v.to_string(),
            Self::u16(v) => v.to_string(),
            Self::u32(v) => v.to_string(),
            Self::f32(v) => v.to_string(),
            Self::f64(v) => v.to_string(),
            Self::bool(v) => v.to_string(),
            Self::String(v) => format!(r#""{v}""#),
            Self::char(v) => format!(r#""{v}""#),
            Self::None => "null".into(),
            _ => "unknown".into(),
        }
    }
}

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

        let t = datatype(config, &ty, &type_map)?;

        s.push_str(&t);

        if t == "null" {
            s.push_str(" = null");
        }
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

/// A wrapper of [`specta::ts::datatype`] to properly format a [`Result`] type.
pub fn datatype(conf: &ExportConfig, typ: &DataType, type_map: &TypeMap) -> Result<String> {
    if let DataType::Result(res) = typ {
        let ok = datatype(conf, &res.0, type_map)?;
        let err = datatype(conf, &res.1, type_map)?;

        Ok(format!("Result<{}, {}>", ok, err))
    } else if let DataType::Nullable(res) = typ {
        let it = datatype(conf, &res, type_map)?;

        Ok(format!("Option<{}>", it))
    } else {
        specta::ts::datatype(conf, typ, type_map)
    }
}
