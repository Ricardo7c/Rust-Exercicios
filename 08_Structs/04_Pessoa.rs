struct Pessoa{
    nome:String,
    idade:u32,
    peso:f32,
    altura:f32
}

impl Pessoa {
    fn envelhecer(&mut self, anos:u32){
        if self.idade <= 21{
            for _ in self.idade..21{
                self.altura += 0.05;
            }
        }
        self.idade += anos;
    }

    fn engordar(&mut self, quilos:f32){
        self.peso += quilos;
    }

    fn emagrecer (&mut self, quilos:f32){
        self.peso -= quilos;
    }

    fn crescer(&mut self){
        self.altura += 0.10;
    }

    fn mostrar_pessoa(&self){
        println!("Nome: {}", self.nome);
        println!("Idade: {}", self.idade);
        println!("Peso: {:.2}", self.peso);
        println!("Altura: {:.2}", self.altura);
    }

}

fn main(){
    let mut p1 = Pessoa{
        nome: String::from("Ricardo"),
        idade: 34,
        peso: 117.8,
        altura: 1.85
    };

    p1.mostrar_pessoa();
    println!("---------------");
    p1.envelhecer(10);
    p1.emagrecer(20.0);
    p1.engordar(5.0);
    p1.crescer();
    p1.mostrar_pessoa();

}