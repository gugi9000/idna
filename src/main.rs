extern crate idna;

use idna::punycode::{decode, encode_str, decode_to_string};

fn main() {
    let dom = "blå";
    let tld = ".dk";
    match encode_str(dom) {
        None => println!("Nope!"),
        Some(i) => println!("xn--{}.{}", i.to_string(),tld)
    }
}
