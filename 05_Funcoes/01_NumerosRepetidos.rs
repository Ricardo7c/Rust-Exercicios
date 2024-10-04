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

fn numeros(x:i32){
    for cada in 0..=x{
        println!(" ");
        for _ in 0..cada{
            print!("{:02}", cada)
        }
    }
}

fn main(){
    let num = int("Digite um numero: ");
    numeros(num);
}