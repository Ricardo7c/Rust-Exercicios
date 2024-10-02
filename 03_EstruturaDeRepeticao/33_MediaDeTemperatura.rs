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
    let mut temp:f32;
    let mut maior = 0.0;
    let mut menor = 0.0;
    let media:f32;
    let mut soma = 0.0;
    let mut cont = 0.0;
    loop{
        temp = float("Digite a temperatura ou 0 para sair: ");
        if temp == 0.0{
            break;
        }else if menor == 0.0 || menor > temp {
            menor = temp;
        }else if maior < temp {
            maior = temp
        }
        cont += 1.0;
        soma += temp;
    }

    media = soma/cont;

    println!("A maior temperatura registrada foi: {:.1} ºC", maior);
    println!("A menor temperatura registrada foi: {:.1} ºC", menor);
    println!("A media de temperaturas registradas foi: {:.1} ºC", media);
}