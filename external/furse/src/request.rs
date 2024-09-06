use std::collections::HashMap;

use crate::{api_calls::Response, Furse, Result};
use once_cell::sync::Lazy;
use reqwest::{IntoUrl, Url};
use serde::{de::DeserializeOwned, Serialize};

pub(crate) static API_URL_BASE: Lazy<Url> =
    Lazy::new(|| Url::parse("https://api.curseforge.com/v1/").unwrap());

impl Furse {
    /// Perform a GET request to `url` and deserialise to `T`
    pub(crate) async fn get<T>(&self, url: impl IntoUrl) -> Result<Response<T>>
    where
        T: DeserializeOwned,
    {
        Ok(self
            .client
            .get(url)
            .header("x-api-key", &self.api_key)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?)
    }

    /// Perform a GET request to `url` with a query and deserialise to `T`
    pub(crate) async fn get_with_query<T>(
        &self,
        url: impl IntoUrl,
        query: HashMap<&str, String>,
    ) -> Result<Response<T>>
    where
        T: DeserializeOwned,
    {
        let query = query
            .iter()
            .map(|v| (v.0.to_string(), v.1.to_owned()))
            .collect::<Vec<_>>();

        Ok(self
            .client
            .get(url)
            .header("x-api-key", &self.api_key)
            .query(query.as_slice())
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?)
    }

    /// Perform a GET request to `url` with `body`
    pub(crate) async fn post<T, B>(&self, url: impl IntoUrl, body: &B) -> Result<Response<T>>
    where
        T: DeserializeOwned,
        B: Serialize,
    {
        Ok(self
            .client
            .post(url)
            .json(body)
            .header("x-api-key", &self.api_key)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?)
    }
}
