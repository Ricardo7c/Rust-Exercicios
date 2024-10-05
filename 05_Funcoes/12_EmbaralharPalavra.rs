use std::io::{self, Write};
use rand::{seq::SliceRandom, thread_rng};

fn input(texto: &str) -> String{ 
    print!("{}", texto);
    io::stdout().flush().unwrap();
    let mut x = String::new();
    io::stdin().read_line(&mut x).unwrap();
    x
}

fn embaralhar(palavra:String) -> String{

    //Transforma a palavra em um vetor de caracteres
    let mut caracteres: Vec<char> = palavra.trim().chars().collect();

    // Cria o randomizador
    let mut rng = thread_rng();

    // Usa o embaralhador com o randomizador
    caracteres.shuffle(&mut rng);
    
    // Converte o vetor embaralhado em uma string
    let x = caracteres.iter().collect();

    x

}



fn main(){
    let palavra = input("Digite a palavra para ser embaralhada: ");

    print!("{}",embaralhar(palavra));
}