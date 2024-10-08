struct Macaco{
    nome: String,
    bucho: Vec<String>
}

impl Macaco{
    fn comer(&mut self, comida: Comida){
        match comida {
            Comida::Macaco(outro_macaco) => {
                self.bucho.extend(outro_macaco.bucho.clone());
                self.bucho.push(outro_macaco.nome);
            }
            Comida::ComidaSimples(comida_simples) => {
                self.bucho.push(comida_simples);
            }
        }
    }

    fn exibir_bucho(&self){
        println!("O bucho do {} tem: {:?}", self.nome, self.bucho)
    }


}

enum Comida{
    Macaco(Macaco),
    ComidaSimples(String),
}

fn main(){
    let mut macaco1 = Macaco{
        nome: String::from("Gorilla"),
        bucho: vec!["Banana".to_string()]
    };

    let mut macaco2 = Macaco{
        nome: String::from("Urango Tango"),
        bucho: vec!["Morango".to_string()]
    };

    macaco1.comer(Comida::ComidaSimples("Leite".to_string()));
    macaco1.exibir_bucho();

    macaco2.comer(Comida::ComidaSimples("Uva".to_string()));
    macaco2.comer(Comida::Macaco(macaco1));
    macaco2.exibir_bucho();
}