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
    let preco = float("Digite o preço do pão: ");

    println!(" ______________________________");
    println!("|-------- Panificadora --------|");
    println!("|                              |");
    for cada in 1..51{
        println!("| {:02} ---------------- R$ {:05.2} |", cada, cada as f32*preco);
    }
    println!("|______________________________|")
}   