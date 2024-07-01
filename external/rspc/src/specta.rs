//! This file is the magic that allows switching Specta versions.

use std::borrow::Cow;

use specta::{DataType, internal::construct::sid, LiteralType, Type, TypeMap};

use crate::internal::ProcedureDataType;

pub fn typedef<TArg: Type, TResult: Type>(type_map: &mut TypeMap) -> ProcedureDataType {
    let arg_ty = TArg::reference(type_map, &[]).inner;
    let result_ty = TResult::reference(type_map, &[]).inner;
    let arg_ty_name = arg_ty.type_name();
    let result_ty_name = result_ty.type_name();
    let arg_ty = arg_ty.to_named(arg_ty_name.clone());
    let result_ty = result_ty.to_named(result_ty_name.clone());
    let arg_name = arg_ty_name.as_ref().to_owned();
    let result_name = result_ty_name.as_ref().to_owned();

    type_map.insert(sid((Box::leak(Box::new(arg_name))).as_str(), ""), arg_ty.clone());
    type_map.insert(sid((Box::leak(Box::new(result_name))).as_str(), ""), result_ty.clone());

    ProcedureDataType { arg_ty, result_ty }
}

pub trait TypeName {
    fn type_name(&self) -> Cow<'static, str>;
}

impl TypeName for LiteralType {
    fn type_name(&self) -> Cow<'static, str> {
        match self {
            LiteralType::i8(_) => "i8".into(),
            LiteralType::i16(_) => "i16".into(),
            LiteralType::i32(_) => "i32".into(),
            LiteralType::u8(_) => "u8".into(),
            LiteralType::u16(_) => "u16".into(),
            LiteralType::u32(_) => "u32".into(),
            LiteralType::f32(_) => "f32".into(),
            LiteralType::f64(_) => "f64".into(),
            LiteralType::bool(_) => "bool".into(),
            LiteralType::String(_) => "String".into(),
            LiteralType::char(_) => "char".into(),
            _ => "None".into(),
        }
    }
}

impl TypeName for DataType {
    fn type_name(&self) -> Cow<'static, str> {
        match self {
            DataType::Any => "any".into(),
            DataType::Enum(e) => e.name().clone(),
            DataType::Generic(g) => format!("{}", g).into(),
            DataType::List(l) => format!("Vec<{}>", l.ty().type_name()).into(),
            DataType::Literal(l) => l.type_name().into(),
            DataType::Map(m) => format!(
                "HashMap<{}, {}>",
                m.key_ty().type_name(),
                m.value_ty().type_name()
            )
            .into(),
            DataType::Nullable(t) => format!("Option<{}>", t.type_name()).into(),
            DataType::Primitive(p) => p.to_rust_str().into(),
            DataType::Reference(r) => r.name().clone(),
            DataType::Result(r) => {
                format!("Result<{}, {}>", r.0.type_name(), r.1.type_name()).into()
            }
            DataType::Struct(s) => s.name().clone(),
            DataType::Tuple(t) => format!(
                "({})",
                t.elements()
                    .iter()
                    .map(|v| v.type_name())
                    .collect::<Vec<_>>()
                    .join(", ")
            )
            .into(),
            DataType::Unknown => "unknown".into(),
        }
    }
}
