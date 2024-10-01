use std::io::{self, Write};

fn int(texto: &str) -> i32{
    loop{
        print!("{}", texto);
        io::stdout().flush().unwrap();
        let mut x = String::new();
        io::stdin().read_line(&mut x).unwrap();
        match x.trim().parse::<i32>() {
            Ok(x) => return x,
            Err(_) => println!("Valor invalido!")
        }
    }
}

fn main(){
    let mut soma = 0;
    let mut cont = 0;
    loop {
        let idade = int("Digite uma idade ou 00 para sair: ");
        if idade == 00{
            break;
        }else{
            soma += idade;
            cont += 1;
        }
    }

    let media = soma/cont;

    match media {
        0..=25 => println!("A turma é jovem"),
        26..=60 => println!("A turma é adulta"),
        _ => println!("A tuma é Idosa")
    }

}