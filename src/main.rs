extern crate whatlang;

use whatlang::{detect, Lang, Script};

fn main() {
    let text = "Bonjour à tous, j'espère que vous allez bien !";
    let info = detect(text).unwrap();
    println!("{}", info.lang());
    println!("{}", info.confidence());
}
