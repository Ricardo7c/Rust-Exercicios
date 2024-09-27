use std::io::{self, Write};

fn main(){
    print!("Qual sua altura? ");
    io::stdout()
        .flush()
        .expect("");
    let mut altura = String::new();
    io::stdin()
        .read_line(&mut altura)
        .expect("");
    let altura:f32 = altura.trim()
        .parse()
        .expect("");
    print!("Você é (H)omen ou (M)ulher? ");
    io::stdout()
        .flush()
        .expect("");
    let mut sexo = String::new();
    io::stdin()
        .read_line(&mut sexo)
        .expect("");
    let sexo = sexo.trim()
        .to_uppercase();

    match sexo.as_str(){
        "H" => println!("Seu peso ideal é: {:.2}kg", (72.7*altura)-58.0),
        "M" => println!("Seu peso ideal é: {:.2}kg", (62.1*altura)-44.7),
        _ => println!("Valor invalido!"),
    }
}   