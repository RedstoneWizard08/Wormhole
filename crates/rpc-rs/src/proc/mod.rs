use std::future::Future;

use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};
use serde_json::Error;

use crate::util::TripleS;

#[async_trait]
pub trait GenericProcedure<Cx: Send + Sync> {
    async fn run(&self, cx: Cx, data: String) -> Result<String, Error>;
}

#[async_trait]
impl<
        Cx: TripleS + Clone,
        Output: Send + Sync + Serialize,
        Arg: Send + Sync + 'static + DeserializeOwned,
    > GenericProcedure<Cx> for WrappedProcedure<Cx, Output, Arg>
{
    async fn run(&self, cx: Cx, data: String) -> Result<String, Error> {
        serde_json::to_string(&self.0.exec(cx, serde_json::from_str(&data)?).await)
    }
}

pub struct WrappedProcedure<
    Cx: TripleS + Clone,
    Output: Send + Sync + Serialize,
    Arg: Send + Sync + 'static + DeserializeOwned,
>(Box<dyn Procedure<Cx, Output, Arg> + Send + Sync>);

#[async_trait]
pub trait Procedure<
    Cx: TripleS + Clone,
    Output: Send + Sync + Serialize,
    Arg: Send + Sync + DeserializeOwned,
>
{
    async fn exec(&self, cx: Cx, data: Arg) -> Output;
}

#[async_trait]
impl<
        Cx: TripleS + Clone,
        Output: Send + Sync + Serialize,
        Fut: Future<Output = Output> + Send + Sync,
        Arg: Send + Sync + 'static + DeserializeOwned,
        Func: (Fn(Cx, Arg) -> Fut) + Send + Sync + 'static,
    > Procedure<Cx, Output, Arg> for Func
{
    async fn exec(&self, cx: Cx, data: Arg) -> Output {
        self(cx, data).await
    }
}

pub fn wrap<
    Cx: TripleS + Clone,
    Output: Send + Sync + Serialize,
    Arg: Send + Sync + 'static + DeserializeOwned,
    Proc: Procedure<Cx, Output, Arg> + Send + Sync + 'static,
>(
    proc: Proc,
) -> WrappedProcedure<Cx, Output, Arg> {
    WrappedProcedure(Box::new(proc))
}
