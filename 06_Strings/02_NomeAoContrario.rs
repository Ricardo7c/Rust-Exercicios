use std::io::{self, Write};

fn input(texto: &str) -> String{
    print!("{}",texto);
    io::stdout().flush().unwrap();
    let mut x = String::new();
    io::stdin().read_line(&mut x).unwrap();
    x
}

fn main(){

    let nome = input("Digite o seu nome: ").to_uppercase();
    let mut nome_reverso = String::new();
    for cada in nome.chars().rev(){
        nome_reverso.push(cada);
    };
    println!("{}", nome_reverso);
}