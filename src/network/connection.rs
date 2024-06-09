use std::sync::Arc;
use std::time::Duration;

use futures::{SinkExt, StreamExt};
use log::trace;
use serde_json::json;
use tokio::io::WriteHalf;
use tokio::net::TcpStream;
use tokio::sync::mpsc::Sender;
use tokio::sync::Mutex;
use tokio_util::codec::{FramedRead, FramedWrite};

use crate::network::io::data::Data;
use crate::network::io::io::AppCodec;

#[derive(Debug, Clone)]
pub struct Connection {
    pub running: bool,
    pub transport_tx: Option<Arc<Mutex<FramedWrite<WriteHalf<TcpStream>, AppCodec>>>>,
    pub count: u32,
}

impl Connection {
    pub async fn new() -> anyhow::Result<Arc<Mutex<Self>>> {
        let conn = Connection {
            running: true,
            transport_tx: None,
            count: 0,
        };
        let a = Arc::new(Mutex::new(conn));
        Self::connect(a.clone()).await?;
        Ok(a)
    }

    fn set_transport_tx(&mut self, tx: Arc<Mutex<FramedWrite<WriteHalf<TcpStream>, AppCodec>>>) {
        self.transport_tx = Some(tx);
    }

    pub async fn send(&mut self, data: Data) {
        println!("send data: {}", data.data);
        let v1 = data.to_bytes().await;
        let v2: &[u8] = &v1;
        if let Some(tx) = &self.transport_tx {
            let mut tx = tx.lock().await;
            match tx.send(v2).await {
                Ok(_) => {}
                Err(err) => { println!("Error send data to server: {:?}", err) }
            };
            drop(tx);
        } else {
            println!("Error send data to server: transport_tx is None");
        }
    }

    async fn on_data_received(&mut self, data: Data) {
        println!("Data received: {:?}", data);
        if data.service == 1 {
            let ping = json!({
                "requestId": "1",
                "hbc": "1",
            });
            let str = serde_json::to_string(&ping).unwrap();
            let dt = Data { service: 1, data: str };
            self.send(dt).await;
        }
        if data.service == 2 {
            let json: serde_json::Value = serde_json::from_str(&data.data).unwrap();
            if let Some(r) = json["r"].as_i64() {
                if r != 0 {
                    self.count += 1;
                    self.auth().await;
                } else {
                    self.count = 0;
                }
            } else {
                self.count += 1;
                self.auth().await;
            }
        }
    }

    async fn auth(&mut self) {
        let mut user;
        if self.count >= 10 {
            user = json!({
                        "requestId": "1",
                        "username": "username",
                        "password": "password",
                        "module_name": "client",
                        "dc_id": "dc_id",
                        "ip": "127.0.0.1",
                        "port": "9999",
                    });
        } else {
            user = json!({
                        "requestId": "1",
                        "username": "u",
                        "password": "password",
                        "module_name": "client",
                        "dc_id": "dc_id",
                        "ip": "127.0.0.1",
                        "port": "9999",
                    });
        }
        // Serialize the JSON object to a string
        let json_string = serde_json::to_string(&user).unwrap();
        let dt = Data { service: 2, data: json_string };
        self.send(dt).await;
    }

    async fn connect(mut conn: Arc<Mutex<Connection>>) -> anyhow::Result<()> {
        tokio::task::spawn(async move {
            loop {
                if !conn.lock().await.running {
                    break;
                }
                match TcpStream::connect("45.119.83.244:4444").await {
                    Ok(stream) => {
                        println!("Connected to the server");

                        let esl_codec = AppCodec {};
                        let (read_half, write_half) = tokio::io::split(stream);

                        let mut transport_rx = FramedRead::new(read_half, esl_codec.clone());
                        let transport_tx = Arc::new(Mutex::new(FramedWrite::new(write_half, esl_codec.clone())));

                        let mut cn = conn.lock().await;
                        // set sender
                        cn.set_transport_tx(transport_tx.clone());
                        // authentication
                        cn.auth().await;
                        drop(cn);

                        loop {
                            if let (Some(Ok(dt))) = transport_rx.next().await {
                                println!("start read data from server");
                                let mut cn = conn.lock().await;
                                println!("lock connection");
                                cn.on_data_received(dt.clone()).await;
                                println!("on_data_received");
                                drop(cn);
                                println!("drop");
                            } else {
                                println!("Error reading from the server");
                                break;
                            }
                        }
                    }
                    Err(err) => {
                        println!("Error connecting to the server: {:?}", err);
                    }
                }
                println!("Reconnecting in 5 secs");
                let secs_5 = Duration::from_secs(5);
                tokio::time::sleep(secs_5).await;
            }
        });
        Ok(())
    }
}

#[tokio::test]
async fn it_should_be_connect_success() -> anyhow::Result<()> {
    // start connection
    let mut connection = Connection::new().await?;
    // listen for an interruption
    println!("start connection");
    tokio::time::sleep(Duration::from_secs(5)).await;
    let ping = json!({
        "requestId": "1123",
        "module_name": "client",
    });
    let str = serde_json::to_string(&ping).unwrap();
    let dt = Data { service: 3, data: str };
    let mut cn = connection.lock().await;
    cn.send(dt).await;
    drop(cn);
    println!("send message to server");
    tokio::signal::ctrl_c().await.unwrap();
    Ok(())
}