use std::io::{self, Write};

fn input_float(texto: &str) -> f32{
    print!("{}",texto);
    io::stdout().flush().unwrap();
    let mut x = String::new();
    io::stdin().read_line(&mut x).unwrap();
    let x:f32 = x.trim().parse().unwrap();
    x
}

fn main(){
    let nota1 =input_float("Digite a primeira nota: ");
    let nota2 =input_float("Digite a segunda nota: ");
    let nota3 =input_float("Digite a terceira nota: ");

    let media = (nota1+nota2+nota3)/3.0;
    let msg:String;

    if media < 7.0{
        msg = String::from("REPROVADO");
    }
    else if media >= 7.0 && media < 10.0{
        msg = String::from("APROVADO");
    }else{
        msg = String::from("APROVADO COM DESTINÇÃO");
    }
    println!("O aluno teve media {:.1} e foi {}", media, msg);
}