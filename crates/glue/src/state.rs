// MIT License
//
// Copyright (c) 2024 Jacob Sapoznikow <jacob1coder@gmail.com>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE. 

use anyhow::Result;
use http_body_util::Full;
use hyper::{
    body::{Bytes, Incoming},
    client::conn::http1::{handshake, SendRequest},
    header::{HeaderValue, HOST},
    HeaderMap, Method, Request, Response, Uri,
};
use hyper_util::rt::TokioIo;
use tokio::{net::TcpStream, spawn};

use crate::{framework::Framework, util::scheme_port};

#[derive(Debug, Clone)]
pub struct ProxyState {
    /// The base URL. Must be `http(s)://[domain]:[port]`.
    /// This MUST not have a trailing slash.
    pub(crate) base: String,

    /// The framework.
    pub(crate) framework: Framework,
}

impl ProxyState {
    pub fn new<T>(base: T, framework: Framework) -> Self
    where
        T: AsRef<str>,
    {
        Self {
            base: base.as_ref().to_string(),
            framework,
        }
    }

    pub(crate) async fn create_sender(&self) -> Result<SendRequest<Full<Bytes>>> {
        let url = self.base.parse::<Uri>()?;
        let host = url.host().ok_or(anyhow!("Cannot get host from URL!"))?;

        let port = url
            .port_u16()
            .unwrap_or(scheme_port(url.scheme_str().unwrap_or("http")));

        let addr = format!("{}:{}", host, port);
        let stream = TcpStream::connect(addr).await?;
        let io = TokioIo::new(stream);
        let (sender, conn) = handshake(io).await?;

        spawn(async move {
            if let Err(err) = conn.await {
                error!("Connection failed: {:?}", err);
            }
        });

        Ok(sender)
    }

    pub async fn request<T>(
        &self,
        method: Method,
        url: T,
        query: Option<Vec<(String, String)>>,
        body: Option<Bytes>,
        headers: Option<HeaderMap<HeaderValue>>,
    ) -> Result<Response<Incoming>>
    where
        T: AsRef<str>,
    {
        let mut sender = self.create_sender().await?;
        let mut url = url.as_ref().split('?').nth(0).unwrap().to_string();

        if let Some(query) = query {
            let mut buf = String::new();

            for (key, val) in query {
                let part = format!("{}={}", key, val);

                if buf.is_empty() {
                    buf.push_str(&format!("?{}", part));
                } else {
                    buf.push_str(&format!("&{}", part));
                }
            }

            url = format!("{}{}", url, buf);
        }

        let abs_url = format!("{}{}", self.base, url).parse::<Uri>()?;

        let authority = abs_url
            .authority()
            .ok_or(anyhow!("Cannot get authority from URL!"))?
            .clone();

        let mut builder = Request::builder()
            .uri(url)
            .method(method)
            .header(HOST, authority.as_str());

        if let Some(headers) = headers {
            for (key, value) in headers {
                if let Some(key) = key {
                    builder = builder.header(key, value);
                }
            }
        }

        let body = body.map(Full::new).unwrap_or(Full::new(Bytes::new()));

        let req = builder.body(body)?;

        Ok(sender.send_request(req).await?)
    }
}
