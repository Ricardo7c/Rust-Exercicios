use std::io::{self, Write};

fn float(texto:&str, valor:&str) -> f32{
    loop {
        print!("{}{}: ",texto, valor);
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
    let meses = ["Janeiro", "Fevereiro", "Março", "Abril", "Maio", "Junho", "Julho", "Agosto", "Setembro", "Outubro", "Novembro", "Dezembro"];
    let mut temperaturas = Vec::new();
    let mut soma = 0.0;

    for mes in meses{
        let temp = float("Digite a temperatura do mês de ", mes);
        temperaturas.push(temp);
        soma += temp;
    }  

    let media = soma/12.0;

    println!("------------------------------");
    println!("A temperatura media anual foi: {:.1}", media);
    println!("Os meses que ultrapassaram a media anual foram: ");
    for indice in 0..meses.len(){
        if temperaturas[indice] > media{
            println!("{} - {}", meses[indice], temperaturas[indice]);
        }
    }
}