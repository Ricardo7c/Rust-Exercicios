use std::io::{self, Write};

fn input(texto: &str) -> String{ 
    print!("{}", texto);
    io::stdout().flush().unwrap();
    let mut x = String::new();
    io::stdin().read_line(&mut x).unwrap();
    x
}


fn validador_de_data(texto: &str) -> (i32, &str, i32){
    loop{
        let data = input(texto);
        let dia:i32;
        let mes:i32;
        let ano:i32;
        let bisexto:bool;
        let mut dias_do_mes = 28;
        let meses = vec!["Janeiro","Fevereiro","Março","Abril","Maio","Junho","Julho","Agosto","Setembro","Outubro","Novembro","Dezembro"];
        if data.trim().len() != 10 || &data[2..3] != "/" || &data[5..6] != "/" || &data[0..=1] == "00" || &data[3..=4] == "00"{
            println!("A data é invalida!");
        }else{
            dia = data[0..2].parse::<i32>().unwrap();
            mes = data[3..5].parse::<i32>().unwrap();
            ano = data[6..10].parse::<i32>().unwrap();
            bisexto = (ano % 400 == 0) || (ano % 4 == 0 && ano % 100 != 0);
            match mes {
                2 => if bisexto{ dias_do_mes = 29},
                1|3|5|7|8|10|12 => dias_do_mes = 31,
                4|6|9|11 => dias_do_mes = 30,
                _ => {println!("A data é invalida"); continue;},
            }
            
            let dataformatada = (dia, meses[(mes-1)as usize], ano);
            if dia <= dias_do_mes && mes <= 12{
                return dataformatada
            }else{
                println!("A data é invalida!");
                continue;
            }   
        }
    }
}   

fn main(){
    let (dia, mes, ano) = validador_de_data("Digite a dada(dd/mm/aaa): ");
    print!("{} de {} de {}", dia, mes, ano);
}