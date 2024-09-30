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
    let base = int("Digite o numero base: ");
    let expoente = int("Digite o expoente: ");

    let mut resultado = base;
    for _ in 1..expoente{
        resultado *= base;
    }

    if expoente < 0{
        resultado = 1/resultado;
    }

    println!("{} elevado a {} é: {}", base, expoente, resultado);
}