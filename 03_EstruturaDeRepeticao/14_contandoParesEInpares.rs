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
    let mut par = 0;
    let mut impar = 0;
    let mut num:i32;
    for _ in 0..10{
        num = int("Digite um numero: ");
        if num % 2 == 0{
            par += 1;
        }else{
            impar += 1
        }
    }

    println!("Foram digitados {} numeros pares, e {} numeros impares", par, impar);
}