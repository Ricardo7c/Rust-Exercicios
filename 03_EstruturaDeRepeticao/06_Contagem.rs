// Faça um programa que imprima na tela os números de 1 a 20, um abaixo do outro.
// Depois modifique o programa para que ele mostre os números reversos um ao lado do outro.

fn main(){
    for cada in 1..21{
        println!("{}",cada);
    }

    for cada in (1..21).rev(){
        print!("{} ", cada)
    }

}