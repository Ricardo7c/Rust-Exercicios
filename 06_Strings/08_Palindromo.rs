use std::io::{self, Write};

fn input(texto: &str) -> String{ 
    print!("{}", texto);
    io::stdout().flush().unwrap();
    let mut x = String::new();
    io::stdin().read_line(&mut x).unwrap();
    x
}

fn main(){
    let texto = input("Digite seu nome: ").to_uppercase();
    let mut original = String::new();
    let mut reverso = String::new();
    for letra in texto.trim().chars(){
        if letra != ' ' && letra != ',' && letra != '-'{
            original.push(letra);
        }
    }

    for letra in original.trim().chars().rev(){
        reverso.push(letra);
    }

    println!("{}",original);
    println!("{}", reverso);

    if original == reverso{
        println!("é um palindromo!");
    }else{
        println!("não é um palindromo");
    }
}