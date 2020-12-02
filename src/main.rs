extern crate whatlang;

use whatlang::{detect, Lang};

fn main() {
    let text = "Ĉu vi ne volas eklerni Esperanton?";
    let info = detect(text).unwrap();
    let info = info.lang();

    match info {
        Lang::Eng => println!("English"),
        Lang::Fra => println!("French"),
        _ => println!("Please enter text in English or French."),
    }
}
