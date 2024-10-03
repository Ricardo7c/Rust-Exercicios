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
    let mut medias = Vec::new();
    for aluno in 1..=10{
        println!("Digite as notas do {}º aluno", aluno);
        let mut soma = 0.0;
        for _ in 1..=4{
            let nota = float("Digite a nota: ");
            soma += nota;
        }
        medias.push(soma/4.0);
    }
    let mut cont = 0;
    for nota in medias{
        if nota > 7.0{
            cont += 1;
        }
    }

    println!("{} alunos tiverem media maior ou igual a 7.0", cont);




}