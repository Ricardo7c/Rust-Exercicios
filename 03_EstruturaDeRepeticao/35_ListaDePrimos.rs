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
    let numero = int("Digite um numero: ");
    let mut primo:bool;
    let mut lista_primos = Vec::new();

    for num in 2..numero{
        primo = true;
        if num < 2 || (num > 2 && num % 2 == 0){
            primo = false;
        }else{
            for cada in 2..num{
                if num % cada == 0{
                    primo = false;
                }
            }
        }
    
        if primo{
            lista_primos.push(num);
        }
    }

    println!("{:?}", lista_primos);
}