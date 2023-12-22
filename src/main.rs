fn main() {

    let number: i32 = 8;


    if number % 4 == 0 {
        println!("Le nombre est divisible par 4");
    } else if number % 3 == 0 {
        println!("Le nombre est divisible par 3");
    } else if number % 2 == 0 {
        println!("Le nombre est divisible par 2");
    } else {
        println!("Le nombre n'est pas divisible par 4, 3 ou 2");
    }
}

fn variable_if() {
    let condition = true;
    let nombre = if condition { 5 } else { 6 };

    println!("La valeur du nombre est : {}", nombre);
}