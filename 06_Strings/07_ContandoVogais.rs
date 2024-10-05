use std::io::{self, Write};

fn input(texto: &str) -> String{ 
    print!("{}", texto);
    io::stdout().flush().unwrap();
    let mut x = String::new();
    io::stdin().read_line(&mut x).unwrap();
    x
}

fn main(){
    let texto = input("Digite um texto: ");

    let mut vogal = 0;
    let mut espaco = 0;
    for cada in texto.to_uppercase().trim().chars(){
        match cada {
            'A'|'E'|'I'|'O'|'U' => vogal += 1,
            ' ' => espaco += 1,
            _ => continue
        }
    };

    println!("Vogais: {}", vogal);
    println!("Espaços: {}", espaco);

}