use std::io::{self, Write};

fn int(texto:&str) -> i32{
    loop{
        print!("{}",texto);
        io::stdout().flush().unwrap();
        let mut x = String::new();
        io::stdin().read_line(&mut x).unwrap();
        match x.trim().parse::<i32>() {
            Ok(x) => return x,
            Err(_) => println!("Valor invalido!")
        }
    }
}

fn main(){
    let num = int("Digite um numero: ");
    let mut primos = Vec::new();
    primos.push(2);
    for cada in (3..=num).step_by(2){
        primos.push(cada);
    }

    let mut divisores = 0;
    for (_, numero) in primos.clone().into_iter().enumerate(){
        if numero > 2{
            for cada in 2..=((numero as f32).sqrt() as i32){
                divisores += 1;
                if numero % cada == 0{
                    if let Some(pos) = primos.iter().position(|&x| x == numero) {
                        primos.remove(pos);
                    }
                }
            }
        }
    }

    print!("Os numeros primos entre 1 e {num} são: ");

    for cada in primos{
        print!("{} ", cada)
    }
    println!("");
    println!("O total de divisores: {}", divisores)

}