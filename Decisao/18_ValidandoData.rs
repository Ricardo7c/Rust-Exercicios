// Faça um Programa que peça uma data no formato dd/mm/aaaa e determine se a mesma é uma data válida.

use std::io::{self, Write};

fn input_data(texto: &str) -> Vec<i32>{
    loop{
        print!("{}",texto);
        io::stdout().flush().unwrap();
        let mut x = String::new();
        io::stdin().read_line(&mut x).unwrap();
        let vetor_string:Vec<String> = x.trim().split('/').map(|s| s.to_string()).collect();
        let x : Vec<i32> = vetor_string.iter().map(|s|s.parse::<i32>().unwrap()).collect();
        if x.len() == 3{
            return x;
        }else {
            println!("Valor invalido!")
        }
    }
}

fn main(){
    let data = input_data("Digite a data (dd/mm/aaaa): ");
    let dia = &data[0];
    let mes = &data[1];
    let ano = &data[2];
    let meses31 = [1, 3, 5, 7, 8, 10, 12];
    let dias_ranged:i32;

    if *mes == 02{
        if ano % 400 == 0 || ano % 4 == 0{
            dias_ranged = 29;
        }else{
            dias_ranged = 28;
        }
    }else if meses31.contains(mes){
        dias_ranged = 31;
    }else{
        dias_ranged = 30;
    }

    if *dia <= dias_ranged && *mes <= 12{
        println!("A data {}/{}/{} é valida!", dia, mes, ano);
    }else{
        println!("A data {}/{}/{} é invalida!", dia, mes, ano);
    }
}
