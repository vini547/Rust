//Loops sao feitos para iterar até uma condição seja alcançada
pub fn run() {
    let mut c = 0;

    //Infinite Lopp
    loop {
        c +=1;
        println!("N: {}", c);

        if c == 20 {
            break;
        }
    }

    while c <=100 {
        if c % 5 == 0 {
            println!("Divisível por 5");
        } else if c % 3 == 0 {
            println!("Divisível por 3");
        } else if c % 2 == 0 {
            println!(" Divisível po 2");
        } else {
            println!("{}", c);
        }
        c +=1;
    }

    //For Range
    for x in 0..100 {
        if x % 5 == 0 {
            println!("Divisível por 5");
        } else if x % 3 == 0 {
            println!("Divisível por 3");
        } else if x % 2 == 0 {
            println!(" Divisível po 2");
        } else {
            println!("{}", x);
        }
        
    }

}