
const ARR:[usize;5] = [10,20,30,40,50];


pub fn if_else_conditions() {

    let number:i32 = 8;


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

pub fn variable_if() {
    let condition = true;
    let nombre = if condition { 5 } else { 6 };

    println!("La valeur du nombre est : {}", nombre);
}


pub fn infinite_loop(){
    loop {
        println!("ahhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhh");
        //commenter le break pour la rendre infinie
        break
    }
}


pub fn compteur(){
    let mut compteur:i32 = 0;

    'increment :loop {
        println!("compteur: {}", compteur);
        let mut restant:i32 = 10;

        loop {
            println!("restant: {}", restant);
            if compteur == 3 {
                break 'increment
            }

            if restant == 9{
                break
            }

            restant -= 1
        }
        compteur += 1
    }
    println!("Le compteur est de {}",compteur)
}

pub fn return_result_in_loop(){
    let mut compteur : i32 = 0;

    let result = loop {

        compteur+=1;

        if compteur == 10 {
            break compteur*2
        }

    };
    println!("Compteur : {}",result)
}

pub fn while_loop_train(){

    let mut counter :i32 = 3;

    while counter > 0 {
        println!("{}",counter);
        counter -= 1;
    }
    println!("YEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEES")
}

pub fn while_inside_an_array(){

    let mut index  = 0;

    while index < 5 {
        println!("array[{}] = {}",index,ARR[index]);
        index += 1;
    }
}

pub fn for_loop_training(){


    let mut index:i32 = 0;

    for element in ARR {
        println!("array[{}] = {}",index,element);
        index +=1;
    }

}

fn for_loop_with_rev(){
    for number in (1..4).rev() {
        println!("{}!",number)
    }
    println!("Yessssssssssssssssssss")
}



fn main() {
    for_loop_with_rev()
}