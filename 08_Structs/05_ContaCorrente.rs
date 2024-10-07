struct ContaCorrente{
    num: i32,
    nome: String,
    saldo: f32,
}

impl ContaCorrente {
    fn alterar_nome(&mut self, novonome:String){
        self.nome = novonome;
    }

    fn deposito(&mut self, novosaldo:f32){
        println!("Depositado com sucesso!");
        self.saldo += novosaldo;
    }

    fn saque(&mut self, novosaldo:f32){
        println!("Saque efetuado!");
        self.saldo -= novosaldo;
    }

    fn mostrar_saldo(&self){
        println!("Saldo: {}", self.saldo);
    }
    fn mostrar_dados(&self){
        println!("Numero: {}", self.num);
        println!("Nome: {}", self.nome);
        println!("Saldo: {}", self.saldo);
    }

}

fn main(){
    let mut cc = ContaCorrente {
            num: 1,
            nome: String::from("Ricardo"),
            saldo: 68.90
            };

    cc.mostrar_saldo();
    cc.deposito(25.00);
    cc.mostrar_saldo();
    cc.alterar_nome(String::from("Pedro"));
    cc.saque(5.00);
    println!("-----------------------");
    cc.mostrar_dados();
}