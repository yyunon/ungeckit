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
    use tokio::net::TcpStream;
    use log::*;
    use tokio_tungstenite::{self, MaybeTlsStream, WebSocketStream};
    use std::sync::{Arc, Mutex};
    use crate::service::Context;
    use crate::utils::error::GeckError;
    use futures_util::{SinkExt, StreamExt};
    use tokio::sync::mpsc;
    use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
    // TODO Move CDP in here!!!
    pub struct WebSocketClient {
        pub context: Arc<Mutex<Context>>,
        ws_url: String,
        ws_stream: Option<WebSocketStream<MaybeTlsStream<TcpStream>>>,
        rx: mpsc::UnboundedReceiver<Message>,
        tx: mpsc::UnboundedSender<Message>,
    }

    impl WebSocketClient {
        pub fn new(context: Arc<Mutex<Context>>, ws_url: &str) -> Self {
            let ch = mpsc::unbounded_channel();
            Self {
                context: context,
                ws_url: ws_url.to_owned(),
                ws_stream: None,
                tx: ch.0,
                rx: ch.1,
            }
        }

        pub async fn connect_async(&mut self) {
            let (tx, rx) = (&mut self.tx, &self.rx) ;
            let (ws_stream, _) = connect_async(self.ws_url.clone()).await.expect("Cannot create connection");
            self.ws_stream = Some(ws_stream);
            debug!("Successfully connected to the websocket stream");
        }

        pub fn connect(&mut self) {
            let ctx = self.context.clone();
            ctx.lock().unwrap().handle.block_on(async move {
                let (ws_stream, _) = connect_async(self.ws_url.clone()).await.expect("Cannot create connection");
                self.ws_stream = Some(ws_stream);
            });
            debug!("Successfully connected to the websocket stream");
        }

        pub fn send(&mut self, msg: &str) -> Result<Message, GeckError> {
            let ctx = self.context.clone();
            let sc = self.ws_stream.as_mut().unwrap();
            let parsed_msg = ctx.lock().unwrap().handle.block_on(async move {
                match sc.send(msg.into()).await {
                    Ok(data) => {log::debug!("Message is sent successfully {:?}", data)}
                    Err(e) => log::error!("{:?}", e)
                }
                // Handle Remote Agent errors
                if let Some(message) = sc.next().await {
                    let data = message.unwrap();
                    log::debug!("Response message is: {:?}", data);
                    return Some(data);
                } 
                None
            });
            assert!(parsed_msg != None, "WebSocketClient couldn't retrieve the response message");
            Ok(parsed_msg.unwrap())
        }

    }

}