use crate::config::{Config, IConfig};
use mongodb::sync::Client;
pub struct Connection;

pub trait IConnection {
    fn init(&self) -> Client;
}

impl IConnection for Connection {
    fn init(&self) -> Client {
        let config: Config = Config {};
        let username: String = config.get_config_with_key("DATABASE_USER");
        let password: String = config.get_config_with_key("DATABASE_PASSWORD");
        let client: Client = Client::with_uri_str(&format!("mongodb+srv://{}:{}@cluster0.euyes.mongodb.net", username,password)).unwrap();
        client
    }
}