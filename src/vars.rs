//VAriaveis carregam dados ou referências a dados
//Variaveis são immutaveis by default, vc nã pode reaasing como na linha 9 do codigo abaixo
//Rust é uma linguagem block scoped, ou seja, se vc chama uma variavel detro de uma funçãoela pertence à este escopo.
#![allow(non_snake_case)]

pub fn run() {
    let name = "Karl";
    let age = 99;

   //age = 38



   //DEFINIDNO CONSTATNTES
   const ID: i32 = 001;

   //DEFININDO MÚLTIPLAS VARIÁVEUS DE UMA SÓ VEZ

   let (a, b) = ("Karl", 98);
 

   println!("ID ist       >>>==|> {}", ID);
   println!("Der name ist >>>==|> {}", name);
   println!("{}, {}", a, b);

}