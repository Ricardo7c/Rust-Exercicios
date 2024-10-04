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


fn verificar(x:i32) -> &'static str{
    if x > 0{
        return "POSITIVO";
    }else{
        return "NEGATIVO";
    }
}

fn main(){
    let num = int("Digite um numero: ");
    let verificado = verificar(num);
    println!("{}", verificado);
}