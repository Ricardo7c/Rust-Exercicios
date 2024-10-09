use std::fs::File;
use std::io::prelude::Read;

fn ler_arquivo(arquivo: &str) -> String{
    let mut x = String::new();
    File::open(arquivo).unwrap().read_to_string(&mut x).unwrap();
    x
}


fn main() {
    println!("{}", ler_arquivo("novo.txt"));
}