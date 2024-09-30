use std::io::{self, Write};

fn input(texto: &str) -> String{
    print!("{}", texto);
    io::stdout().flush().unwrap();
    let mut x = String::new();
    io::stdin().read_line(&mut x).unwrap();
    x
}

fn int(texto: &str) -> i32{
    loop {
        let valor = input (texto);
        match valor.trim().parse::<i32>() {
            Ok(valor) => return valor,
            Err(_) => println!("Valor invalido, tente novamente!")
        }
    }
}

fn main(){
    let mut maior = 0;
    let mut menor = 0;
    let mut soma = 0;

    loop {
        let num = int("Digite um numero: ");
        if num > 1000 || num < 0{
            println!("Valor invalido, tente novamente!");
        }else{
            soma += num;

            if num > maior{
                maior = num;
            }else if num < menor || menor == 0 {
                menor = num;
            }

            let res = input("Quer continuar S/N? ");
            if res.trim().to_uppercase() == "N".to_string(){
                break;
            }
        }
    }

    println!("O maior foi: {}", maior);
    println!("O menor foi: {}", menor);
    println!("A soma dos numeros foi: {}", soma);
}