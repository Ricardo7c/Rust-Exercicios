use std::io::{self, Write};

fn int(texto:&str) -> i32{
    loop {
        print!("{}", texto);
        io::stdout().flush().unwrap();
        let mut x = String::new();
        io::stdin().read_line(&mut x).unwrap();
        match x.trim().parse::<i32>() {
            Ok(x) => return x,
            Err(_) => println!("Valor invalido!")
        }
    }
}

fn main(){
    let num = int("Digite um numero para ver sua tabuada: ");
    for cada in 1..=10{
        let resultado = num*cada;
        print!("{} x {} = {}", num, cada, resultado);
        println!();
    }
}