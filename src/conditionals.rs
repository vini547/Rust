// Conditionals verificam condições 
pub fn run() {

    let age = 18;
    let check_id = true;

    if age >= 21 && check_id{
        println!("Idade acima de 21,
    permitido consumo");
    } else if age < 21 && check_id{ 
        println!("Idade Menor que 21, 
    acesso negado");
    } else {
        println!("Show you Pass!")
    }

    //Shorthand IF
    let is_of_age = if age >= 21 {true} else {false};
    println!("Is of age: {}", is_of_age);
    
    
}