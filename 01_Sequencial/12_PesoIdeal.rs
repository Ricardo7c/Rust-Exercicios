use std::io::{self, Write};

fn main(){
    print!("Qual sua altura: ");
    io::stdout()
        .flush()
        .expect("");
    let mut altura = String::new();
    io::stdin()
        .read_line(&mut altura)
        .expect("");
    let altura:f32 = altura.trim()
        .parse()
        .expect("");
    let ideal = (72.7*altura)-58.0;

    println!("Seu peso ideal é: {:.2}kg",ideal);

}