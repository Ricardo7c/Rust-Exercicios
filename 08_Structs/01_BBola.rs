struct Bola{
    cor: String,
    circunferencia: f64,
    material: String,
}

impl Bola{
    fn trocar_cor(&mut self, cor:String){
        self.cor = cor
    }

    fn mostar_cor(&self) -> &String{
        return &self.cor
    }
}

fn main(){
    let mut b1 = Bola{
        cor: String::from("Amarela"),
        circunferencia: 23.0,
        material: String::from("Plastico")
    };

    println!("{}", b1.mostar_cor());
    b1.trocar_cor("Cinza".to_string());
    println!("{}", b1.mostar_cor());
    println!("{}", b1.circunferencia);
    println!("{}", b1.material);
}