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

fn int(texto:&str) -> i32{
    loop {
        print!("{}",texto);
        io::stdout().flush().unwrap();
        let mut x = String::new();
        io::stdin().read_line(&mut x).unwrap();
        match x.trim().parse::<i32>() {
            Ok(x) => return x,
            Err(_) => println!("Valor invalido, tente novamente!")
        }
    }
}


fn main(){
    let mut idades = Vec::new();
    let mut alturas = Vec::new();

    for _ in 0..=1{
        let idade = int("Digite sua idade: ");
        let altura = float("Digite sua altura: ");
        idades.push(idade);
        alturas.push(altura);
        println!("Cadastro salvo!");
        println!("---------------");
    };

    idades.reverse();
    alturas.reverse();

    println!("Cadastros na ordem reversa: ");
    for cada in 0..idades.len(){
        println!("{} anos - altura: {:.2}", &idades[cada], &alturas[cada]);
    }

}