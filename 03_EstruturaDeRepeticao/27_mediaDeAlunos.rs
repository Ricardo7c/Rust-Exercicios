use std::io::{self, Write};

fn input(texto: &str) -> String{
    print!("{}", texto);
    io::stdout().flush().unwrap();
    let mut x = String::new();
    io::stdin().read_line(&mut x).unwrap();
    x
}

fn float(texto: &str) -> f32{
    loop{
        let x = input(texto);
        match x.trim().parse::<f32>() {
            Ok(x) => return x,
            Err(_) => println!("Valor invalido!")
        }
    }
}

fn main(){
    let turmas = float("Quantas turmas são? ");
    let mut soma = 0.0;
    let mut cont = turmas;
    while cont > 0.0 {
        let alunos = float("A turma tem quantos alunos? ");
        if alunos > 40.0{
            println!("Uma turma não pode ter mais de 40 alunos");
        }else{
            soma += alunos;
            cont -= 1.0;
        }    
    }
    let media = soma / turmas;

    println!("A numero medio de alunos por turma é: {}",media);
}