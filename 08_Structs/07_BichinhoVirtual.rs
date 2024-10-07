struct Tamagoshi{
    nome: String,
    fome: u32,
    saude: u32,
    idade: u32,
}

impl Tamagoshi {
    fn alterar_nome(&mut self, nome: String){
        self.nome = nome;
    }

    fn alterar_fome(&mut self, fome: u32){
        self.fome = fome;
    }

    fn alterar_saude(&mut self, saude: u32){
        self.saude = saude;
    }

    fn alterar_idade(&mut self, idade: u32){
        self.idade = idade;
    }

    fn humor(&self) -> String{
        let humor = (self.saude+self.fome)/2;
        match humor {
            80..=100 => return "Feliz".to_string(),
            50..80 => return "Tranquilo".to_string(),
            30..50 => return "Chatiado".to_string(),
            _ => return "Mal humorado".to_string()
        }
    }

    fn mostrar_status(&self){
        println!("Nome: {}", self.nome);
        println!("Fome: {}", self.fome);
        println!("Saude: {}", self.saude);
        println!("Idade: {}", self.idade);
        println!("Humor: {}", self.humor());
        println!("------------------")
    }
}

fn main(){
    let mut x = Tamagoshi{
        nome: String::from("Wally"),
        idade: 5,
        fome: 55,
        saude: 60
    };

    x.mostrar_status();
    x.alterar_nome("Pedro".to_string());
    x.alterar_fome(20);
    x.alterar_saude(21);
    x.alterar_idade(12);
    x.mostrar_status();
}
