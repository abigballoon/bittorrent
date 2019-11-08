pub struct MetaInfo {
  pub info_hash: Vec<u8>,
}

impl MetaInfo {
  pub fn new() -> MetaInfo {
    MetaInfo {
      info_hash: vec![0; 20],
    }
  }
}