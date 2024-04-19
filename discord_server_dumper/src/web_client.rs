use reqwest::{Client, header};

pub struct Token {
    pub token: String,
}

impl Token {
    pub fn new_token(token: &str) -> Self{
        Self{
            token: token.to_owned(),
        }
    }
    pub async fn http(client: &Client, url: String) -> Option<String> {
        println!("{url}");
        let x = client.get(url).send().await.unwrap().text().await.unwrap();
        Some(x)
    }
    pub fn new_web_client(self) -> Client {
        let webclient = Client::builder();
        let mut headers = header::HeaderMap::new();
        headers.insert(header::AUTHORIZATION, header::HeaderValue::from_str(self.token.as_str()).unwrap());
        webclient.default_headers(headers).build().unwrap()
    }
}
