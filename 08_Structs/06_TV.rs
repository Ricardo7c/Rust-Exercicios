struct Tv{
    volume: u32,
    canal: u32
}

impl Tv {
    fn trocar_canal(&mut self, canal:u32){
        if canal <= 200{
            self.canal = canal;
        }else{
            println!("Canal invalido!");
        }
    }

    fn aumentar_volume(&mut self){
        if self.volume < 100{
            self.volume += 1;
        }else{
            println!("Volume maximo!")
        }
    }

    fn diminuir_volume(&mut self){
        if self.volume < 100{
            self.volume -= 1;
        }else{
            println!("Mudo ativado!")
        }
    }

    fn mostrar_tv(&self){
        println!("Canal {} | Volume: {}", self.canal, self.volume);
        println!("----------------------");
    }

}

fn main(){
    let mut tv = Tv{
        volume: 13,
        canal: 33
    };

    tv.mostrar_tv();
    tv.diminuir_volume();
    tv.trocar_canal(43);
    tv.mostrar_tv();
    tv.aumentar_volume();
    tv.mostrar_tv();



}