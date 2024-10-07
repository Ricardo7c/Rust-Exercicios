struct Quadrado {
    lado: i32
}

impl Quadrado {
    fn mudar_lado(&mut self, lado:i32) {
        self.lado = lado;
    }

    fn mostrar_lado(&self) -> i32{
        return self.lado
    }

    fn calcular_area(&mut self) -> i32{
        let area = self.lado * self.lado;
        return area
    }
}

fn main(){
    let mut q1 = Quadrado{
        lado: 2,
    };

    println!("Mostrar lado: {}", q1.mostrar_lado());
    println!("Calcular Area: {}", q1.calcular_area());
    q1.mudar_lado(5);
    println!("Mostrar lado: {}", q1.mostrar_lado());
    println!("Calcular Area: {}", q1.calcular_area());

}