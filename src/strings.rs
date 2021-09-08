#![allow(non_snake_case)]
//EXISTEM DOIS TIPOS DE STRING NO RUST
//Primitive str = Imutavel string de tamanho fixo em um lugar qualquer da memoria
//String        = Estrutura de dados que pode crescer e que é alocada no heap. Utilizada quando se precisa modificar os dados da string

pub fn run() {
  //Cria uma simples str
  let hell = "Hello";
  println!("A variável hell e: {}", hell);

  //Cria uma String heap-alocated etc..
  let mut hello = String::from("Hello ");
  println!("{}", hello);

  //Conseguir o tamanho da str
  println!("Tamanho da String e: {}", hello.len());

  //Adicionando um único char à variavel hello usando o método push
  hello.push('W');
  println!("{}", hello);

  //Adicionando mais de um char, ou seja, uma str, utiliza-se o método push_str
  hello.push_str("orld");
  println!("{}", hello);

  //Conseguir tamanho da String
  println!("Tamanho da String e: {}", hello.len());

  //Quatidade em bytes utilizada pela variavel hello
  println!("Utilização em bytes da variável hello e: {}", hello.capacity());

  //CHecar se a varivável está vazia
  println!("Is empty? {}", hello.is_empty());

  //CHecar se a varivável contém a palavra hello(Diferencia maiuscula de minuscula
  println!("Contém a palavra hello? {}
", hello.contains("Hello"));

  //Trocar uma palavra específica por uma outra qualquer definida. Repaare que dados da variavel não mudam
  println!("Trocando palavra

WORLD por BROW 
HELLO por WHAT'S_UP

==> {} , {}", hello.replace("World", "BROW"), hello.replace("Hello", "WHAT'S_UP"));
  println!("{}", hello);



}