use std::io::prelude::*;
use std::net::TcpStream;
use uuid::Uuid;

pub struct Client {
    pub current_host: String,
    pub stream: TcpStream,
    pub buff_size: usize,
    pub session_id: String,
    pub player_name: String,
}

impl Client {
    pub fn create_client(host: String, buff_size: i32) -> std::io::Result<Self> {
        let stream = TcpStream::connect(host.clone())?;

        Ok(Self {
            current_host : host.clone(),
            stream: stream,
            buff_size: buff_size as usize,
            player_name: "".into(),
            session_id: Uuid::new_v4().to_string(),
        })
    }

    pub fn send(&mut self, data: String) -> std::io::Result<()> {
        self.stream.write(&data.as_bytes())?;
        Ok(())
    }

    pub fn recv(&mut self) -> std::io::Result<String> {
        let mut buff = vec![0_u8; self.buff_size];
        self.stream.read(&mut buff)?;
        Ok(String::from_utf8(buff.to_vec()).unwrap())
    }

    pub fn to_string(&self) -> String {
        let str_data = format!(r#"
TCP Client Struct Data:
            Player Name: {}
            Current Host: {}
            Session ID: {}
            Buffer Size: {}
        "#, self.player_name, self.current_host, self.session_id, self.buff_size);

        str_data
    }
}