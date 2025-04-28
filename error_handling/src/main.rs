#![allow(unused)]

fn main() {
    use std::fs::File;
    use std::io::{self, Read};
    
    fn lire_pseudo_depuis_fichier() -> Result<String, io::Error> {
        let f = File::open("hello.txt");
    
        let mut f = match f {
            Ok(fichier) => fichier,
            Err(e) => return Err(e),
        };
    
        let mut s = String::new();
    
        match f.read_to_string(&mut s) {
            Ok(_) => Ok(s),
            Err(e) => Err(e),
        }
    }

    lire_pseudo_depuis_fichier();
}
