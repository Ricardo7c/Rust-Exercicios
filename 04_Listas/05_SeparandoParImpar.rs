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
    let mut todos = Vec::new();
    let mut par = Vec::new();
    let mut impar = Vec::new();

    for _ in 0..20{
        let num = int("Digite um numero: ");
        todos.push(num);
        if num % 2 == 0{
            par.push(num);
        }else{
            impar.push(num);
        }
    }
    println!(" ");
    println!("Todos os numeros: {:?}", todos);
    println!("Numeros pares: {:?}", par);
    println!("Numeros impares: {:?}", impar);
}