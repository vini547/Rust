//funcções são usadas para guardarblocos de codigo para posterior utilização
pub fn run() {
   greeting("Hello", "Jane");

   //Bind valor de uma função à uma variavel
   let get_sum = add(5, 5);
   println!("Sum: {}", get_sum);

   //Closure
   let n3: i32 = 10;
   let add_num = |n1: i32, n2: i32| n1 + n2 + n3;
   println!("C Sum {}", add_num(3, 3));
}

pub fn greeting(greet: &str, name :&str) {
  println!("{} {}, WIE GHETS?!", greet, name);
}

pub fn add(n1: i32, n2: i32) -> i32 {
    n1 + n2
}