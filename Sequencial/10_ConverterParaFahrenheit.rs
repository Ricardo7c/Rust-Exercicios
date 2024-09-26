use std::io::{self, Write};

fn main(){
    print!("Qual a temperatura em Celcios? ");
    io::stdout()
        .flush()
        .expect("Falha ao limpar");

    let mut temp = String::new();
    io::stdin()
        .read_line(&mut temp)
        .expect("Valor invalido!");

    let temp:f32 = temp.trim()
        .parse::<f32>()
        .expect("");

    let faren = 32.0+(temp*1.8);
    println!("{}° Celcios equivalem a {:.2}° Fahrenheit", temp, faren)





}