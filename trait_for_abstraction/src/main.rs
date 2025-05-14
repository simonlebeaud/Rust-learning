use trait_for_abstraction::{Bouton, Ecran};

use trait_for_abstraction::Affichable;

struct ListeDeroulante {
    largeur: u32,
    hauteur: u32,
    options: Vec<String>,
}

impl Affichable for ListeDeroulante {
    fn afficher(&self) {
        // code servant à afficher vraiment une liste déroulante
    }
}

fn main() {
    let ecran = Ecran {
        composants: vec![
            Box::new(ListeDeroulante {
                largeur: 75,
                hauteur: 10,
                options: vec![
                    String::from("Oui"),
                    String::from("Peut-être"),
                    String::from("Non"),
                ],
            }),
            Box::new(Bouton {
                largeur: 50,
                hauteur: 10,
                libelle: String::from("OK"),
            }),
        ],
    };

    ecran.executer();
}