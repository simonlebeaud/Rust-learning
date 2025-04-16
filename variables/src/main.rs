// const TROIS_HEURES_EN_SECONDES: u32 = 60 * 60 * 3;

fn main() {
    let x = 5;

    let x = x + 1;
    {
        let x = x * 2;
        println!("La valeur de x dans la portée interne est : {}", x);
    }
    
    println!("La valeur de x est : {}", x);
}
