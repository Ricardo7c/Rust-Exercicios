use std::io::{self, Write};

fn float_input(texto: &str) -> f32{
    print!("{}",texto);
    io::stdout().flush().unwrap();
    let mut x = String::new();
    io::stdin().read_line(&mut x).unwrap();
    let x :f32 = x.trim().parse().unwrap();
    x
}

fn main(){
    loop {
        let nota = float_input("Digite uma nota valida entre 1/10: ");
        if nota >= 0.0 && nota <= 10.0{
            println!("{} é uma nota valida!", nota);
            break;
        }else{
            println!("Valor invalido, tente novamente!");
            println!("________________________________");
        }
    }
}