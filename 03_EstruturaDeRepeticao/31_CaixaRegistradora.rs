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
    loop{
        let mut lista = Vec::new();
        let mut soma = 0.0;
        let mut cont = 1;
        loop {
            let compra = float("Digite o valor do produto ou 0 para sair: ");
            if compra == 0.0 {
                break;
            }else{
                lista.push(compra);
            }
        }

        if lista.is_empty(){
            break;
        }

        for cada in lista{
            soma += cada;
            cont += 1;
            println!("Produto {}: R$ {:05.2}", cont, cada);
        }

        println!("Total: R$ {:05.2}", soma);
        let dinheiro = float("Dinheiro: R$ ");
        println!("Troco: R$ {:05.2}", dinheiro-soma);
        println!("----- Compra finalizada -----");
        println!(" ");

    }

}