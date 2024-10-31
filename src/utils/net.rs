use bytes::Bytes;
use reqwest::{self, header, Method, Request, RequestBuilder, Url};

use crate::utils::error::{ErrorKind, GeckError};
pub mod http {
    use super::*;

    pub async fn request<'a>(
        client: &reqwest::Client,
        verb: &str,
        url: &str,
        data: String,
    ) -> Result<Bytes, GeckError> {
        let method = Method::from_bytes(verb.as_bytes()).unwrap();
        let mut headers = header::HeaderMap::new();
        headers.insert("Accept", "application/json".parse().unwrap());
        headers.insert(
            "Content-Type",
            "application/json;charset=UTF-8".parse().unwrap(),
        );
        let request = Request::new(method, Url::parse(&url).expect("Cannot parse url"));
        let response = RequestBuilder::from_parts(client.clone(), request)
            .headers(headers)
            .body(data)
            .send()
            .await?;
        if response.status() == 200 {
            Ok(response.bytes().await?)
        } else {
            Err(GeckError::new(
                ErrorKind::Service,
                None::<GeckError>,
                &format!("Response status is {}", response.status()),
            ))
        }
    }

}

pub mod ws {
    use futures::prelude::*;
    use futures_util::stream::{SplitSink, SplitStream};
    use serde::Deserialize;
    use tokio::net::TcpStream;
    use log::*;
    use tokio_tungstenite::{self, MaybeTlsStream, WebSocketStream};
    use std::sync::{Arc, Mutex};
    use crate::service::Context;
    use futures_util::{future, pin_mut, SinkExt, StreamExt};
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::sync::mpsc;
    use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
    // TODO Move CDP in here!!!
    pub struct WebSocketClient {
        pub context: Arc<Mutex<Context>>,
        session_id: String,
        ws_url: String,
        id: i32,
        write: Option<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>>,
        rx: mpsc::UnboundedReceiver<Message>,
        tx: mpsc::UnboundedSender<Message>,
    }

    impl WebSocketClient {
        pub fn new(context: Arc<Mutex<Context>>, session_id: &str, ws_url: &str) -> Self {
            let ch = mpsc::unbounded_channel();
            Self {
                context: context,
                session_id: session_id.to_owned(),
                ws_url: ws_url.to_owned(),
                id: 0,
                write:None,
                tx: ch.0,
                rx: ch.1,
            }
        }

        pub async fn connect_async(&mut self) {
            let (tx, rx) = (&mut self.tx, &self.rx) ;
            let (ws_stream, _) = connect_async(self.ws_url.clone()).await.expect("Cannot create connection");
            let (write, read) = ws_stream.split();
            self.write = Some(write);
            let hand = tokio::runtime::Handle::try_current().unwrap();
            hand.spawn( async move {
                read.for_each(|message| async move {
                    let data = message.unwrap();
                    //log::debug!("{:?}", data);
                }).await;
            });
            debug!("Successfully connected to the websocket stream");
        }

        pub fn connect(&mut self) {
            let ctx = self.context.clone();
            ctx.lock().unwrap().handle.block_on(async move {
                let (ws_stream, _) = connect_async(self.ws_url.clone()).await.expect("Cannot create connection");
                let (write, read) = ws_stream.split();
                self.write = Some(write);
                let hand = tokio::runtime::Handle::try_current().unwrap();
                hand.spawn( async move {
                    read.for_each(|message| async move {
                        let data = message.unwrap();
                        log::debug!("{:?}", data);
                    }).await;
                });
            });
            debug!("Successfully connected to the websocket stream");
        }

        pub fn send(&mut self, msg: &str) {
            let write = self.write.as_mut().unwrap();
            self.context.lock().unwrap().handle.block_on(async move {
                match write.send(msg.into()).await {
                    Ok(_) => {}
                    Err(e) => log::error!("{:?}", e)
                }
            });
        }

        pub fn send_all(&mut self, msgs: Vec<String>) {
            let write = self.write.as_mut().unwrap();
            self.context.lock().unwrap().handle.block_on(async move {
                for msg in msgs {
                    match write.send(msg.into()).await {
                        Ok(data) => {log::debug!("{:?}", data)}
                        Err(e) => log::error!("{:?}", e)
                    }
                }
            });

        }
    }

}