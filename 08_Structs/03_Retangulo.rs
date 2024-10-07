use std::io::{self, Write};
struct Retangulo{
    ladoa: f32,
    ladob: f32
}

impl Retangulo {
    fn mudar_lados(&mut self, ladoa:f32, ladob:f32){
        self.ladoa = ladoa;
        self.ladob = ladob;
    }

    fn mostrar_lados(&self){
        println!("Comprimento: {}", &self.ladob);
        println!("Largura: {}", &self.ladoa);
    }

    fn calcular_area(&self) -> f32{
        let area = self.ladoa*self.ladob;
        return area
    }

    fn calcular_perimetro(&self) -> f32{
        let perimetro = (self.ladoa+self.ladob)*2.0;
        return perimetro
    }
}

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
    let comprimento = float("Digite o comprimento: ");
    let largura = float("Digite a largura: ");

    let mut local = Retangulo{
        ladoa: largura,
        ladob: comprimento
    };

    local.mostrar_lados();
    println!("Sera nescessario: {} m2 de Piso", local.calcular_area());
    println!("e {} m de Rodapé", local.calcular_perimetro());
    local.mudar_lados(3.0, 6.0);
    local.mostrar_lados();
    println!("Sera nescessario: {} m2 de Piso", local.calcular_area());
    println!("e {} m de Rodapé", local.calcular_perimetro());
}