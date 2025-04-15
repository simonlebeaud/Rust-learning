use std::io;

fn main() {
    println!("Deviner le nombre !");

    println!("Veuiller entrer un nombre");

    let mut supposition = String::new();

    io::stdin()
        .read_line(&mut supposition)
        .expect("Echech de la lecture de l'entrée utilisateur");

    println!("Votre nombre : {}", supposition);
}
