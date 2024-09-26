use std::io::{self, Write};

fn input(texto:&str) -> String{
    print!("{texto}");
    io::stdout()
        .flush()
        .expect("");
    
    let mut x = String::new();
    io::stdin()
        .read_line(&mut x)
        .expect("");

    x

}

fn float(valor: String) -> f32{
    let x = valor.trim()
        .parse()
        .expect("");
    x
}

fn main(){
    let num1 = float(input("Digite um numero inteiro: "));
    let num2 = float(input("Digite outro numero inteiro: "));
    let real = float(input("Digite um numero real: "));

    println!("O Produto do dobro de {} com a metade de {} é: {}", num1, num2, ((num1*2.0)*(num2/2.0)));
    println!("A soma do triplo de {} com {} é: {}", num1, real, real+(num1*3.0));
    println!("{} elevado ao cubo é: {:.2}", real, real.powf(3.0));
}