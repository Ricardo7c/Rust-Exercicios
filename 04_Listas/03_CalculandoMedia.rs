use std::io::{self, Write};

fn float(texto:&str) -> f32{
    loop {
        print!("{}",texto);
        io::stdout().flush().unwrap();
        let mut x = String::new();
        io::stdin().read_line(&mut x).unwrap();
        match x.trim().parse::<f32>() {
            Ok(x) => return x,
            Err(_) => println!("Valor invalido, tente novamente!")
        }
    }
}

fn main(){
    let mut notas = Vec::new();

    for nota in 0..4{
        let nota = float("Digita a nota: ");
        notas.push(nota);
    }

    let mut soma = 0.0;
    println!("------- Notas ------");
    for (indice, nota) in notas.iter().enumerate(){
        println!("{}ª nota = {}", indice+1, nota);
        soma += nota;
    }

    println!(" ");
    println!("Media = {:.1}", soma/4.0);
}