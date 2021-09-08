
use std::mem;

pub fn run() {
    let mut n: Vec<i32> = vec![0; 1];  
      



  //Adicionanod ao vetor  
  n.push(5);
  
  //Reasing values
  //n[2] =1024;

  //Get single value
  println!("Single Value da posição n[0]: {:?}", n[0]);
  println!("Comprimento de n: {}", n.len());
  
  println!("{:?}", n);
  
  //Vectors são allocadas no stack
  println!("Esta vector ocupa {} bytes.", mem::size_of_val(&n)); //std::mem::size_of_val(&n)) se transforma em mem::size_of_val(&n) quando delcaramos use std::mem; no nicio do codigo

  //Pegar uma fatia da vector
  //let slc: &[i32] = &n[0..511];
  //println!("Slice: {:?}", slc);

  //Loop entre os valores do vector
  for x in n.iter() {
      println!("Number: {:?}", x);
  }

  //Loop entre os valores e mudando valores
  for x in n.iter_mut() {
      *x *= 2;
    }
  println!("{:?}", n);
}