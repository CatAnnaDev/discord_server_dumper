use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use serde::{Serialize, Deserialize};

use crate::dm_message::RecursiveDump;
use crate::web_client::Token;

mod message;
mod dm_message;
mod web_client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    let path = PathBuf::new().join("server_channel.txt");
    let msg = File::create("raw_dump.txt").unwrap();

    let data = parer_server(&path)?;
    let client = Token::new_web_client(Token::new_token(""));
    for x in data{
        for y in x.server_channels {
            match y.channel_type.as_str() {
                "Category" => {}
                "Stage" => {}
                _ => {
                    RecursiveDump::start(&RecursiveDump {
                        client: &client,
                        channel_id: &y.channel_id,
                        serv_name: &x.server_name,
                        chan_name: &y.channel_name,
                    },&msg).await;
                }
            }
        }
    }
    Ok(())
}


fn parer_server(path: &PathBuf) -> Result<Vec<ServerMapping>, Box<dyn Error>>{
    let mut tmp = String::new();
    File::open(&path).unwrap().read_to_string(&mut tmp).unwrap();
    let ret = serde_json::from_str::<Vec<ServerMapping>>(&tmp)?;

    Ok(ret)
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ServerMapping {
    pub size: u16,
    pub server_id: String,
    pub server_name: String,
    pub server_channels: Vec<ServerChannel>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ServerChannel {
    pub channel_type: String,
    pub channel_id: String,
    pub channel_name: String,
}
