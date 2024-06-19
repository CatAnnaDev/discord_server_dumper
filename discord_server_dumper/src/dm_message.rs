use std::fs;
use std::fs::File;
use reqwest::Client;
use std::io::Write;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use serde_json::from_str;

use crate::message::MessageStruct;
use crate::web_client::Token;

pub struct RecursiveDump<'a> {
    pub client: &'a Client,
    pub channel_id: &'a String,
    pub serv_name: &'a String,
    pub chan_name: &'a String,
}

impl RecursiveDump<'_> {
    pub async fn start(&self, msg: &File) {
        let mut msg_dump: Vec<String> = vec![];
        let mut last_message_id = String::new();
        let mut tmp_last_message_id = String::new();
        if let Some(txt) = dm_dump_first_request(&self.client, &self.channel_id).await {
            let msg_array = parse_msg(txt, &mut last_message_id, &msg).unwrap();
            msg_dump.extend(msg_array);
            while tmp_last_message_id != last_message_id {
                tmp_last_message_id = last_message_id.clone();
                if let Some(txt) = dm_dump_recursion(&self.client, &self.channel_id, tmp_last_message_id.clone()).await {
                    println!("Nb msg dump: {}", msg_dump.len());
                    let msg_array = parse_msg(txt, &mut last_message_id, &msg).unwrap();
                    msg_dump.extend(msg_array);
                }
            }

            msg_dump.reverse();
            let path = PathBuf::new().join(format!("dump/{}/",&self.serv_name)) ;
            let _ = fs::create_dir_all(&path);
            let end_path = path.join(format!("{}.txt", self.chan_name));
            let mut dump_file = File::create(&end_path).unwrap();
            println!("{}", end_path.display());
            for x in msg_dump {
                write!(dump_file, "{}", x).unwrap();
            }
        }
    }
}

fn parse_msg(txt: String, x1: &mut String, mut dump: &File) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut msg_vec: Vec<String> = vec![];
    write!(dump, "{}\n", txt).unwrap();
    let msg = from_str::<MessageStruct>(&*txt);
    match msg {
        Ok(parse) => {
            let mut atta = String::new();
            for msg in &parse {
                if let Some(att) = &msg.attachments {
                    for m in att {
                        atta.push_str(format!("{}\n", m["url"]).as_str());
                    }
                }
                *x1 = msg.id.clone();
                let x = format!("({}) {}: {} {}\n", msg.timestamp, msg.author.username, msg.content, atta);
                msg_vec.push(x);
                atta.clear();
            }
        }
        Err(ref e) => {
            eprintln!("Error: {e}\n{}", from_str::<ErrorParse>(&*txt).unwrap().message);
        }
    };
    Ok(msg_vec)
}

async fn dm_dump_first_request(client: &Client, channel_id: &str) -> Option<String> {
    Token::http(&client, format!("https://discord.com/api/v9/channels/{}/messages?limit=100", channel_id)).await
}

async fn dm_dump_recursion(client: &Client, channel_id: &str, last_message_id: String) -> Option<String> {
    let url = format!("https://discord.com/api/v9/channels/{}/messages?before={}", channel_id, last_message_id);
    Token::http(&client, url).await
}


#[derive(Serialize, Deserialize)]
pub struct ErrorParse {
    message: String,
    code: i32
}