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
    let num = int("Digite o final da sequencia: ");
    let mut a:i128 = 0;
    let mut b:i128 = 1;
    print!("{} {} ", a, b);
    for _ in 0..num {
        (a, b) = (b, a+b);
        print!("{} ", b);
    }
}