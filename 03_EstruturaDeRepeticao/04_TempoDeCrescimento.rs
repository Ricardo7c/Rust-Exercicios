// Supondo que a população de um país A seja da ordem de 80000 habitantes com uma taxa anual
// de crescimento de 3% e que a população de B seja 200000 habitantes com uma taxa de crescimento
// de 1.5%. Faça um programa que calcule e escreva o número de anos necessários para que a população
// do país A ultrapasse ou iguale a população do país B, mantidas as taxas de crescimento.

fn main(){
    let mut a:f32 = 80000.0;
    let mut b:f32 = 200000.0;
    let mut anos:u8 = 0;

    while a < b {
        a += a*0.03;
        b += b*0.015;
        anos += 1
    }

    println!("Serão nescessarios {} anos", anos);

}