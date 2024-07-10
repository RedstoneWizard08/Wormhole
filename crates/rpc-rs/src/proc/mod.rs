//! The procedure module.

use std::{borrow::Cow, future::Future};

use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};
use serde_json::Error;
use specta::{function::FunctionDataType, Type, TypeMap};

use crate::util::TripleS;

/// A generic procedure. This is required to allow for any procedure
/// to be used in a [`rpc_rs::Module`].
#[async_trait]
pub trait GenericProcedure<Cx: Send + Sync> {
    /// Run this procedure with the context and a JSON-encoded string
    /// representing the data required.
    async fn run(&self, cx: Cx, data: String) -> Result<String, Error>;

    /// Get the [`FunctionDataType`] of this procedure.
    fn specta_type(&self, name: Cow<'static, str>, map: &mut TypeMap) -> FunctionDataType;
}

#[async_trait]
impl<
        Cx: TripleS + Clone,
        Output: Send + Sync + Serialize + Type,
        Arg: Send + Sync + 'static + DeserializeOwned + Type,
    > GenericProcedure<Cx> for WrappedProcedure<Cx, Output, Arg>
{
    async fn run(&self, cx: Cx, mut data: String) -> Result<String, Error> {
        if data.is_empty() {
            data = String::from("null");
        }

        serde_json::to_string(&self.0.exec(cx, serde_json::from_str(&data)?).await)
    }

    fn specta_type(&self, name: Cow<'static, str>, map: &mut TypeMap) -> FunctionDataType {
        self.0.specta_type(name, map)
    }
}

/// A generic-wrapped [`Procedure`].
///
/// This is necessary so the [`GenericProcedure`] trait can be
/// implemented in an object-safe manner.
pub struct WrappedProcedure<
    Cx: TripleS + Clone,
    Output: Send + Sync + Serialize + Type,
    Arg: Send + Sync + 'static + DeserializeOwned + Type,
>(Box<dyn Procedure<Cx, Output, Arg> + Send + Sync>);

/// A procedure.
#[async_trait]
pub trait Procedure<
    Cx: TripleS + Clone,
    Output: Send + Sync + Serialize + Type,
    Arg: Send + Sync + DeserializeOwned + Type,
>
{
    /// Run this procedure with its context and data.
    async fn exec(&self, cx: Cx, data: Arg) -> Output;
}

/// A typed extension to [`Procedure`].
pub trait TypedProcedure<
    Cx: TripleS + Clone,
    Output: Send + Sync + Serialize + Type,
    Arg: Send + Sync + DeserializeOwned + Type,
>: Procedure<Cx, Output, Arg>
{
    /// Get the [`FunctionDataType`] of this procedure.
    fn specta_type(&self, name: Cow<'static, str>, map: &mut TypeMap) -> FunctionDataType;
}

#[async_trait]
impl<
        Cx: TripleS + Clone,
        Output: Send + Sync + Serialize + Type,
        Fut: Future<Output = Output> + Send,
        Arg: Send + Sync + 'static + DeserializeOwned + Type,
        Func: (Fn(Cx, Arg) -> Fut) + Send + Sync + 'static,
    > Procedure<Cx, Output, Arg> for Func
{
    async fn exec(&self, cx: Cx, data: Arg) -> Output {
        self(cx, data).await
    }
}

impl<
        Cx: TripleS + Clone,
        Output: Send + Sync + Serialize + Type,
        Arg: Send + Sync + 'static + DeserializeOwned + Type,
        Proc: Procedure<Cx, Output, Arg> + ?Sized,
    > TypedProcedure<Cx, Output, Arg> for Proc
{
    fn specta_type(&self, name: Cow<'static, str>, map: &mut TypeMap) -> FunctionDataType {
        FunctionDataType {
            args: vec![("data".into(), Arg::reference(map, &[]).inner)],
            asyncness: true,
            deprecated: None,
            name,
            docs: "".into(),
            result: Some(Output::reference(map, &[]).inner),
        }
    }
}

/// Wrap any [`Procedure`], converting it to a [`WrappedProcedure`]
/// which implements the [`GenericProcedure`] trait.
pub fn wrap<
    Cx: TripleS + Clone,
    Output: Send + Sync + Serialize + Type,
    Arg: Send + Sync + 'static + DeserializeOwned + Type,
    Proc: Procedure<Cx, Output, Arg> + Send + Sync + 'static,
>(
    proc: Proc,
) -> WrappedProcedure<Cx, Output, Arg> {
    WrappedProcedure(Box::new(proc))
}
