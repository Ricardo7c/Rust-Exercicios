use std::io::{self, Write};

fn int(texto:&str) -> i32{
    loop{
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
    let num1 = int("Digite o inicio da contagem: ");
    let num2 = int("Digite o final da contagem: ");
    let mut soma = 0;
    if num1 > num2{
        for cada in (num2..=num1).rev(){
            print!("{} ", cada);
            soma += cada;
            }
    }else{
        for cada in num1..=num2{
            print!("{} ", cada);
            soma += cada;
        }
    }
    print!("= {}", soma)
}   

