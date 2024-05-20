// MIT License
//
// Copyright (c) 2024 Jacob Sapoznikow <jacob1coder@gmail.com>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE. 

use axum_core::__composite_rejection as composite_rejection;
use axum_core::__define_rejection as define_rejection;

define_rejection! {
    #[status = METHOD_NOT_ALLOWED]
    #[body = "Request method must be `GET`"]
    /// Rejection type for [`WebSocketUpgrade`](super::WebSocketUpgrade).
    pub struct MethodNotGet;
}

define_rejection! {
    #[status = BAD_REQUEST]
    #[body = "Connection header did not include 'upgrade'"]
    /// Rejection type for [`WebSocketUpgrade`](super::WebSocketUpgrade).
    pub struct InvalidConnectionHeader;
}

define_rejection! {
    #[status = BAD_REQUEST]
    #[body = "`Upgrade` header did not include 'websocket'"]
    /// Rejection type for [`WebSocketUpgrade`](super::WebSocketUpgrade).
    pub struct InvalidUpgradeHeader;
}

define_rejection! {
    #[status = BAD_REQUEST]
    #[body = "`Sec-WebSocket-Version` header did not include '13'"]
    /// Rejection type for [`WebSocketUpgrade`](super::WebSocketUpgrade).
    pub struct InvalidWebSocketVersionHeader;
}

define_rejection! {
    #[status = BAD_REQUEST]
    #[body = "`Sec-WebSocket-Key` header missing"]
    /// Rejection type for [`WebSocketUpgrade`](super::WebSocketUpgrade).
    pub struct WebSocketKeyHeaderMissing;
}

define_rejection! {
    #[status = UPGRADE_REQUIRED]
    #[body = "WebSocket request couldn't be upgraded since no upgrade state was present"]
    /// Rejection type for [`WebSocketUpgrade`](super::WebSocketUpgrade).
    ///
    /// This rejection is returned if the connection cannot be upgraded for example if the
    /// request is HTTP/1.0.
    ///
    /// See [MDN] for more details about connection upgrades.
    ///
    /// [MDN]: https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Upgrade
    pub struct ConnectionNotUpgradable;
}

composite_rejection! {
    /// Rejection used for [`WebSocketUpgrade`](super::WebSocketUpgrade).
    ///
    /// Contains one variant for each way the [`WebSocketUpgrade`](super::WebSocketUpgrade)
    /// extractor can fail.
    pub enum WebSocketUpgradeRejection {
        MethodNotGet,
        InvalidConnectionHeader,
        InvalidUpgradeHeader,
        InvalidWebSocketVersionHeader,
        WebSocketKeyHeaderMissing,
        ConnectionNotUpgradable,
    }
}
