pub mod network {
    use std::net::TcpStream;

    pub fn connect_to_peer(host: &str) -> Option<TcpStream> {
        for port in 6991..7001 {
            println!("trying to connect {}:{}", host, port);
            match TcpStream::connect(format!("{}:{}", host, port)) {
                Ok(result) => {
                    println!("found client at port {}", port);
                    return Some(result);
                }
                Err(_err) => {
                    continue;
                }
            }
        }
        return None;
    }
}

pub mod utils {
    use std::{fmt::Write, num::ParseIntError};
    pub fn string_to_u8(string: &str) -> &[u8] {
        return string.as_bytes();
    }

    pub fn concat_array(a: &[u8], b: &[u8]) -> Vec<u8> {
        return [a, b].concat();
    }

    pub fn decode_hex(s: &str) -> Result<Vec<u8>, ParseIntError> {
        (0..s.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
            .collect()
    }

    pub fn encode_hex(bytes: &[u8]) -> String {
        let mut s = String::with_capacity(bytes.len() * 2);
        for &b in bytes {
            write!(&mut s, "{:02x}", b);
        }
        s
    }
}