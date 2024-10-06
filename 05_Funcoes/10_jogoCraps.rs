use rand::Rng;
use std::io::{self, Write};

fn jogar_dados() -> (i32, i32, i32){
    pause();
    let dado1 = rand::thread_rng().gen_range(1..=6);
    let dado2 = rand::thread_rng().gen_range(1..=6);
    let total = dado1+dado2;
    return (dado1, dado2, total);
}

fn jogada(total:i32, lanc:i32) -> bool{
    let mut ponto:i32 = 0;
    if lanc == 1{
        if total == 7 || total == 11{
            println!("Você é um natural, venceu o jogo!");
            return true
        }else if total == 2 || total == 3 || total == 12{
            println!("CRAPS: Você perdeu!");
            return true
        }else{
            println!("Ponto pra você!");
            ponto = total;
            println!("Pontos: {}", ponto);
            return false
        }
    }else if total == ponto {
        println!("Você venceu!");
        return true
    }else if total == 7{
        println!("Você perdeu!");
        return true
    }else{
        println!("Continue tentando!");
        return false
    }


}

fn pause() {
    let mut input = String::new();
    print!("Pressione Enter para lançar os dados");
    io::stdout().flush().unwrap(); 
    io::stdin().read_line(&mut input).unwrap();
}


fn main(){
    println!("--------- Bem vindo ao craps ---------");
    let mut lanc = 0;
    let mut play = false;
    while play == false {
        let tupla: (i32, i32, i32) = jogar_dados();
        lanc += 1;
        print!("{} e {} total: {} ", tupla.0, tupla.1, tupla.2);
        play = jogada(tupla.2, lanc)
    }
}