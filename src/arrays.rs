//Arrays são listas onde os elementos são do mesmo data type.
use std::mem;

pub fn run() {
 let mut n: [i32; 1024] = [0; 1024];     
  
  //Reasing values
  n[2] =1024;

  //Get single value
  println!("Single Value da posição n[0]: {:?}", n[0]);
  println!("Comprimento de n: {}", n.len());
  
  println!("{:?}", n);
  
  //Arrays são allocadas no stack
  println!("Esta array ocupa {} bytes", mem::size_of_val(&n)); //std::mem::size_of_val(&n)) se transforma em mem::size_of_val(&n) quando delcaramos use std::mem; no nicio do codigo

  //Pegar uma fatia da array
  let slc: &[i32] = &n[0..511];
  println!("Slice: {:?}", slc);

}

