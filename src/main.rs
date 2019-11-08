extern crate rand;

mod common;
mod client;
mod metainfo;

fn generate_peer_id() -> String {
  let mut rng = rand::thread_rng();
  let rand_chars: String = rng.gen_ascii_chars().taks(20).collect();
  format!("{}", rand_chars)
}

fn main() -> std::io::Result<()> {
  let connection = common::network::connect_to_peer("10.212.7.100");
  match connection {
    Some(stream) => {
      println!("enter io");
      let metainfo = metainfo::MetaInfo::new();
      let peer_id = generate_peer_id();
      let mut peer: client::Connection = client::Connection::new(stream, metainfo, "hha".to_string());
      peer.handshake()?;
    }
    None => println!("haha")
  }
  Ok(())
}
