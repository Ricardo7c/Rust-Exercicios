fn main(){
    println!(" ______________________________");
    println!("|------ Lojas Quase Dois ------|");
    println!("|                              |");
    for cada in 1..51{
        println!("| {:02} ---------------- R$ {:05.2} |", cada, cada as f32*1.99);
    }
    println!("|______________________________|")
}   