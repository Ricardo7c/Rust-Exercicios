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
    let num = int("Digite um numero: ");
    let mut primo = true;

    if num < 2 || (num > 2 && num % 2 == 0){
        primo = false;
    }else{
        for cada in 3..num{
            if num % cada == 0{
                primo = false;
            }
        }
    }

    if primo{
        println!("é primo!")
    }else{
        println!("Não é primo!")
    }
}