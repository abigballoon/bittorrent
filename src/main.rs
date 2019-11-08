use std::io::prelude::*;
pub mod common;

fn main() -> std::io::Result<()> {
    let connection = common::network::connect_to_peer("10.212.7.100");
    match connection {
        Some(mut stream) => {
            println!("enter io");
            let mut content: Vec<u8>;
            content = common::utils::concat_array(&[19],
                                                  common::utils::string_to_u8("BitTorrent protocol"));
            content = common::utils::concat_array(&content, &[0, 0, 0, 0, 0, 0, 0 ,0]);
            match common::utils::decode_hex("c9bc228321c467ec197fc9dfd97811002046fec7") {
                Ok(hex) => { content = common::utils::concat_array(&content, &hex); }
                Err(_e) => {}
            }
            match common::utils::decode_hex("41322d312d33342d302d8838db3e46db95ad831e") {
                Ok(hex) => { content = common::utils::concat_array(&content, &hex); }
                Err(_e) => {}
            }
            stream.write(&content)?;

            match stream.read(&mut [0; 128]) {
                Ok(_okresult) => {
                    println!("{}", _okresult);
                },
                Err(_err) => {
                    println!("hello");
                }
            }
        }
        None => println!("haha")
    }
    Ok(())
}