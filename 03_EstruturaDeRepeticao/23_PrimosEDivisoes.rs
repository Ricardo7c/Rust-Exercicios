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
    let mut numeros = vec![2];
    let mut divisores = 0;
    let mut primos_validos :Vec<i32> = Vec::new();

    for cada in (3..=num).step_by(2){
        numeros.push(cada);
    }

    for &num in &numeros{
        if num == 2{
            primos_validos.push(num);
            continue;
        }
        let mut primo = true;
        for cada in 2..=((num as f32).sqrt() as i32){
            divisores += 1;
            if num % cada == 0{
                primo = false;
            }
        }
        if primo{
            primos_validos.push(num);
        }
    }

    print!("Os numeros primos entre 1 e {num} são: ");
    println!("{:?}", primos_validos);
    println!("");
    println!("O total de divisores: {}", divisores);

}