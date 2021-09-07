pub fn run() {
    //PRINTA NO CONSOLE
    println!("____________________");    
    println!("Hi there!");
    // BASIC FORMATING 
    println!("____________________");
    println!("{} é = a {}", 1, "1");
    //POSITIONAL ARGUMENTS
    println!("____________________");
    println!("{0} is from {1} and {0} likes to {2}", "Karl", "SLZ", "Speed-Up");
    //NAMED ARGUMENTS
    println!("____________________");
    println!("{name} likes {car}", name = "Karl", car = "E65AMG");
    //PLACE HOLDER TRAITS 
    println!("____________________");
    println!("Binary: {:b} 
HEXA: {:x} 
OCTAL: {:o}", 10051991, 10051991, 10051991);
//PLACE HOLDER FOR DEBUG TRAIT
    println!("____________________");
    println!("{:?}", (12, true, "hello"));
    //BASIC MATH
    println!("____________________");
    println!("10 + 10 = {} 
10 / 10 = {} ", 10 + 10, 10 / 10);
println!("____________________");

}