use std::{borrow::Cow, future::Future};

use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};
use serde_json::Error;
use specta::{function::FunctionDataType, Type, TypeMap};

use crate::util::TripleS;

#[async_trait]
pub trait GenericProcedure<Cx: Send + Sync> {
    async fn run(&self, cx: Cx, data: String) -> Result<String, Error>;
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

pub struct WrappedProcedure<
    Cx: TripleS + Clone,
    Output: Send + Sync + Serialize + Type,
    Arg: Send + Sync + 'static + DeserializeOwned + Type,
>(Box<dyn Procedure<Cx, Output, Arg> + Send + Sync>);

#[async_trait]
pub trait Procedure<
    Cx: TripleS + Clone,
    Output: Send + Sync + Serialize + Type,
    Arg: Send + Sync + DeserializeOwned + Type,
>
{
    async fn exec(&self, cx: Cx, data: Arg) -> Output;
}

pub trait TypedProcedure<
    Cx: TripleS + Clone,
    Output: Send + Sync + Serialize + Type,
    Arg: Send + Sync + DeserializeOwned + Type,
>: Procedure<Cx, Output, Arg>
{
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
