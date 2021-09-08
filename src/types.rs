//Tipos de primitivos - -
//Integers:  u8, i8, u16, i16, u32, i32, u64, i64, u128, i128 || " i" é o número de bits que a varaiavel utilizará
//Floats:    f32, f64
//BOolean:   (bool)
//Character: (char) || é somente um caracter, diferentemente de uma string que são varias letras
//Tuples:    Variável com variso tipos de primitos
//Arrays:


#![allow(non_snake_case)]
//Rust é uma linguagem staticamente tipificada, ou seja, ela precisa saber semrpe qual o tipo de variável na hora de compilar
//Entretanto, o compilador pode usualmente inferir qual tipo precisamos utilizar baseado no valor que atribuirmos a uma variavel e como nós a utilizamos
// Desta fomra não necessário inferir o tipo de uma variavel ao declara-la

pub fn run() {
 //Default is "i32"
 let x = 1;

 //Default is "f64"
 let y = 2.5;

 //Infeirndo tipo de variavel ao declarar
 let z: i64 = 9876543321;

 //Encontrando valor maximo que a variavel pode guardar
 println!("Máximo tamaho de i8   é:  {}",   std::i8::MAX);
 println!("Máximo tamaho de u8   é:  {}",   std::u8::MAX);
 println!("Máximo tamaho de i16  é:  {}",  std::i16::MAX);
 println!("Máximo tamaho de u16  é:  {}",  std::u16::MAX);
 println!("Máximo tamaho de i32  é:  {}",  std::i32::MAX);
 println!("Máximo tamaho de u32  é:  {}",  std::u32::MAX);
 println!("Máximo tamaho de i64  é:  {}",  std::i64::MAX);
 println!("Máximo tamaho de u64  é:  {}",  std::u64::MAX);
 println!("Máximo tamaho de i128 é:  {}", std::i128::MAX);
 println!("Máximo tamaho de u128 é:  {}", std::u128::MAX);
 println!("");

 //Booleano
 let actv = true;
 
 //Conseguir variavel boolean a partir de uma expressão
 let gtr = 10 > 5;
 let face = '\u{1F600}';

 println!("{:?}", (x, y, z, actv, gtr));
 println!("");
 println!("O valor de x é: {:?}", x);
 println!("O valor de y é: {:?}", y);
 println!("O valor de z é: {:?}", z);
 println!("O valor de actv é: {:?}", actv);
 println!("O valor de gtr é: {:?}", gtr);
 println!("O valor de gtr é: {:?}", face);
 println!("");

 //




}