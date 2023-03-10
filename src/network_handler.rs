use std::io::prelude::*;
use std::{thread, time};
use std::net::TcpStream;
use bevy::prelude::*;
use uuid::Uuid;
use std::str;

use crate::components::{*, self};

const SEND_DELAY: time::Duration = time::Duration::from_millis(10);

#[derive(Resource)]
pub struct Client {
    pub current_host: String,
    pub stream: TcpStream,
    pub buff_size: usize,
    pub user_id: String,
    pub player_name: String,
    pub login_info: String,
}

impl Client {
    /// This function creates a new client object and returns it
    /// 
    /// Arguments:
    /// 
    /// * `host`: The hostname of the server.
    /// * `buff_size`: The size of the buffer to use when reading from the socket.
    /// * `login_info`: String - This is a string that contains the username and password separated by a
    /// colon.
    /// 
    /// Returns:
    /// 
    /// A Result<Self>
    pub fn create_client(host: String, buff_size: i32, login_info: String) -> std::io::Result<Self> {

        let parts:Vec<&str> = login_info.split(SPLIT_CHAR).collect();
        let username = parts[0].to_string();

        let mut stream = TcpStream::connect(host.clone())?;

        Ok(Self {
            current_host : host.clone(),
            stream: stream,
            buff_size: buff_size as usize,
            player_name: username,
            user_id: "".to_string(),
            login_info // clear this later
        })
    }

    pub fn send(&mut self, data: String) -> std::io::Result<()> {
        println!("network_handler::client.send() -> {}", data);
        thread::sleep(SEND_DELAY); // just so we dont have collisions
        self.stream.write(&data.as_bytes())?;
        Ok(())
    }

    pub fn send_cmd(&mut self, cmd: String, data:String) -> std::io::Result<()> {
        let cmd_data = cmd + SPLIT_CHAR + &data;
        println!("network_handler::Client::send_cmd() -> Sending: {}", cmd_data);
        self.stream.write(&(cmd_data).as_bytes())?;
        Ok(())
    }

    pub fn login(&mut self) -> std::io::Result<()> {
        self.send(self.login_info.clone())?;
        let uid = self.recv()?;
        self.user_id = uid;
        self.login_info = "".to_string(); // clean it

        Ok(())
    }

    pub fn recv(&mut self) -> std::io::Result<String> {
        println!("network_handler::client.recv() -> Waiting for response...");
        let mut buff = vec![0_u8; self.buff_size];
        self.stream.read(&mut buff)?;
        let mut out_data = String::from_utf8(buff.to_vec()).expect("network_handler::client.recv() -> Found invalid UTF-8");
        out_data = out_data.trim_matches(char::from(0)).to_string();
        
        println!("network_handler::client.recv() -> Recv: '{}'", out_data);
        
        Ok(out_data)
    }

    pub fn to_string(&self) -> String {
        let str_data = format!(r#"
TCP Client Struct Data:
            Player Name  : {}
            User ID      : {}
            Current Host : {}
            Buffer Size  : {}
        "#, self.player_name, self.user_id, self.current_host, self.buff_size);

        str_data
    }
}