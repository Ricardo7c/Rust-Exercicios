use std::io::{self, Write};

fn int(texto:&str) -> i32{
    loop {
        print!("{}",texto);
        io::stdout().flush().unwrap();
        let mut x = String::new();
        io::stdin().read_line(&mut x).unwrap();
        match x.trim().parse::<i32>() {
            Ok(x) => return x,
            Err(_) => println!("Valor invalido, tente novamente!")
        }
    }
}

fn main(){
    let num = int("Montar tabuada de: ");
    let inicio = int("Começar por: ");
    let fim = int("Terminar em: ");

    println!("Vou montar a tabuada de {} começando por {} e terminando em {}", num, inicio, fim);

    for cada in inicio..=fim{
        println!("{} x {} = {}", num, cada, num*cada);
    }
}