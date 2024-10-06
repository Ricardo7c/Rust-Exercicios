use std::io::{self, Write};

fn int(texto: &str) -> usize{ 
    loop{
        print!("{}", texto);
        io::stdout().flush().unwrap();
        let mut x = String::new();
        io::stdin().read_line(&mut x).unwrap();
        match x.trim().parse::<usize>() {
            Ok(x) => return x,
            Err(_) => println!("Valor invalido!")
        }
    }
}

fn main(){
    let mut num = int("Digite um numero até 99: ");

    let unidades = vec!["Zero","Um", "Dois", "Três", "Quatro", "Cinco", "Seis", "Sete", "Oito", "Nove", "Dez", "Onze", "Doze", "Treze", "Catorze", "Quinze", "Dezeseis", "Dezesete", "Dezoito", "Dezenove"];
    let dezenas = vec!["","Vinte", "Trinta", "Quarenta", "Cinquenta", "Sessenta", "Setenta", "Oitenta", "Noventa"];

    let mut dezena = 0;
    let mut unidade = 0;

    match num {
        0..=19 => println!("{}", unidades[num]),
        20|30|40|50|60|70|80|90 => println!("{}", dezenas[10/10]),
        _ => {
            while num > 20 {
                dezena += 1;
                num -= 10;
            }
            while num > 10 {
                unidade += 1;
                num -= 1;
            }
            println!("{} e {}", dezenas[dezena], unidades[unidade]);
        }
    }
        
    }
