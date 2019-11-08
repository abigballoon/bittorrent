use std::io::prelude::*;
use std::net::TcpStream;

use crate::metainfo;

const PROTOCOL: &str = "BitTorrent protocol";

pub struct Connection {
  connection: TcpStream,
  metainfo: metainfo::MetaInfo,
  peer_id: String,
}

impl Connection {
  pub fn new(conn: TcpStream, metainfo: metainfo::MetaInfo, peer_id: String) -> Connection {
    Connection {
      connection: conn,
      metainfo: metainfo,
      peer_id: peer_id
    }
  }

  pub fn handshake(&mut self) -> Result<(), std::io::Error> {
    let message = {
      let mut message = vec![];
      message.push(PROTOCOL.len() as u8);
      message.extend(PROTOCOL.bytes());
      message.extend(&[0; 8]);
      message.extend(self.metainfo.info_hash.iter());
      message.extend(self.peer_id.bytes());
      message
    };

    self.connection.write(&message)?;

    Ok(())
  }
}