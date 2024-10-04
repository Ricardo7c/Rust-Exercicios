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

fn calular_imposto(valor:f32, imposto:f32) -> f32{
    let valor_imposto = valor*(imposto/100.0);
    let novo_preco = valor+valor_imposto;
    return novo_preco;
}


fn main(){
    let valor = float("Digite o valor do produto: ");
    let imposto = float("Digite a taxa de imposto em %: ");
    let total = calular_imposto(valor, imposto);

    println!("O total a pagar é: R${:.2}", total);
}