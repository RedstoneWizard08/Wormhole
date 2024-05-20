// MIT License
//
// Copyright (c) 2024 Jacob Sapoznikow <jacob1coder@gmail.com>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE. 

use futures_util::{stream::SplitSink, Sink, SinkExt};
use std::{fmt::Debug, net::SocketAddr, ops::ControlFlow};
use tokio_tungstenite::tungstenite::Message;

pub async fn process_message<T, E>(
    socket: &mut SplitSink<T, Message>,
    who: SocketAddr,
    message: Message,
) -> ControlFlow<(), ()>
where
    T: Sink<Message, Error = E>,
    E: Debug,
{
    match message {
        Message::Text(txt) => {
            socket.send(Message::Text(txt)).await.unwrap();
        }

        Message::Binary(bin) => {
            socket.send(Message::Binary(bin)).await.unwrap();
        }

        Message::Close(c) => {
            if let Some(cf) = c.clone() {
                info!(
                    "{} sent close with code {} and reason `{}`.",
                    who, cf.code, cf.reason
                );
            } else {
                info!("{who} somehow sent close message without CloseFrame!");
            }

            socket.send(Message::Close(c)).await.unwrap();
            socket.close().await.unwrap();

            return ControlFlow::Break(());
        }

        Message::Pong(v) => {
            socket.send(Message::Pong(v)).await.unwrap();
        }

        Message::Ping(v) => {
            socket.send(Message::Ping(v)).await.unwrap();
        }

        Message::Frame(_) => unreachable!(),
    }

    ControlFlow::Continue(())
}
