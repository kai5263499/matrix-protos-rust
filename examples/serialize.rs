extern crate matrix_protos_rust;

use matrix_protos_rust::protos::io::EverloopImage;

fn main() { 
    let mut eli = EverloopImage::new();
    eli.set_everloop_length(18);
    println!("eli:{:?}", eli);
}