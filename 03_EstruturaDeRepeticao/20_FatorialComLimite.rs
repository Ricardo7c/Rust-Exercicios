use std::io::{self, Write};

fn int(texto: &str) -> i32{
    loop{
        print!("{}",texto);
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
    loop{
        let num = int("Digite um numero: ");
        if num > 16 || num % 2 != 0{
            println!("Valor invalido, tente novamente!")
        }else{
            print!("{}! = ",num);
            let mut resultado = 1;

            for cada in (1..=num).rev(){
                resultado *= cada;
                if cada != 1{
                    print!("{}.", cada)
                }else{
                    print!("{} ", cada)
                }
            }
            println!("= {}", resultado)
        }
    }
}