use std::io::{self, Write};

fn input(texto: &str) -> String{ 
    print!("{}", texto);
    io::stdout().flush().unwrap();
    let mut x = String::new();
    io::stdin().read_line(&mut x).unwrap();
    x
}

fn contar_digitos(num: &str) -> usize{
    let x = num.trim().len();
    x
}

fn main(){
    let num = input("Digite um numero inteiro: ");
    print!("{} possui {} digitos", num.trim(), contar_digitos(&num));
}