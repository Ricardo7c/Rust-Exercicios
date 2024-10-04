use std::io::{self, Write};

fn input(texto: &str) -> String{ 
    print!("{}", texto);
    io::stdout().flush().unwrap();
    let mut x = String::new();
    io::stdin().read_line(&mut x).unwrap();
    x
}

fn main(){
    let num = input("Digite um numero: ");
    for cada in num.trim().chars().rev(){
        print!("{}", cada)
    }
}