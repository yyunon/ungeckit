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
    use futures_util::stream::SplitSink;
    use tokio::net::TcpStream;
    use log::*;
    use tokio_tungstenite::{self, MaybeTlsStream, WebSocketStream};
    use std::borrow::BorrowMut;
    use std::future::IntoFuture;
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
                tx: ch.0,
                rx: ch.1,
            }
        }

        pub async fn connect_async(&mut self, url: &str) {
            let (tx, rx) = (&mut self.tx, &self.rx) ;
            let (ws_stream, _) = connect_async(url).await.expect("Cannot create connection");
            let (write, read) = ws_stream.split();
        }

        pub fn send(&mut self, msg: &str) {
            let (tx, rx) = (&mut self.tx, &self.rx) ;
            let url: &str = self.ws_url.as_ref();
            self.context.lock().unwrap().handle.block_on(async move {
                let (ws_stream, _) = connect_async(url).await.expect("Cannot create connection");
                debug!("Successfully connected to the websocket stream");
                let (mut write, read) = ws_stream.split();
                let sender_s = write.send(msg.into());
                let writer_s = read.for_each(|message| async {
                    let data = message.unwrap();
                    debug!("{:?}", data);
                });
                tokio::select! {
                    _ = sender_s => {}
                    _ = writer_s => {}
                }
            });

        }
    }

}