struct BombaDeCombustivel{
    tipo: String,
    valor: f32,
    quant: f32
}

impl BombaDeCombustivel {
    fn abastecer_por_valor(&mut self, valor:f32){
        self.quant -= valor/5.0;
        println!("{:.1} lts de {} foram utilizados.", valor/5.0, self.tipo);
        println!("Valor R$ {:.2}", self.quant*valor)
    }

    fn abastecer_por_litro(&mut self, quant: f32){
        if quant < self.quant{
            self.quant -= quant;
            println!("{} lts de {} foram utilizados.", quant, self.tipo);
            println!("Valor R$ {:.2}", self.valor*quant);
        }

    }

    fn alterar_valor(&mut self, valor:f32){
        self.valor = valor;
    }

    fn alterar_combustivel(&mut self, tipo: String){
        self.tipo = tipo;
    }

    fn alterar_quantidade_combustivel(&mut self, quant: f32){
        self.quant = quant;
    }

    fn mostrar_bomba(&self){
        println!("Restaram {} lts de {} na bomba de combustivel", self.quant, self.tipo);
    }

}

fn main(){
    let mut gas = BombaDeCombustivel {
        tipo: "Gasolina".to_string(),
        quant: 200.0,
        valor: 5.0,
    };

    gas.alterar_valor(3.0);
    gas.abastecer_por_litro(2.0);
    gas.alterar_quantidade_combustivel(25.0);
    gas.alterar_combustivel("Alcool".to_string());
    gas.abastecer_por_valor(26.30);
    gas.mostrar_bomba();
}