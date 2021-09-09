// Structs - São usadas parra criar dados customizados
///Typical Struct
//struct Color {
  //  red: u8,
  //  green: u8,
  //  blue:u8
//}

//Tuple Struct
//struct Color(u8, u8, u8);

struct Person {
    f_name: String,
    l_name: String
}

impl Person {
    // Construct person
    fn new(first: &str, last: &str) -> Person {
        Person {
            f_name: first.to_string(),
            l_name: last.to_string() 
        }
    }

    //GET FULL NAME
    fn full(&self) -> String {
        format!("{} {}", self.f_name, self.l_name)
    }

    //Set Last name
    fn set_last_name(&mut self, last: &str) {
        self.l_name = last.to_string();
    }

    //NAME TO TUPLE
    fn to_tuple(self) -> (String, String) {
        (self.f_name, self.l_name)

    }
}

pub fn run() {
    //let mut c = Color{
      //  red:   255, 
     //   green: 0,
     //   blue:  0
   // };
   // c.red = 200;
    //println!("Color: {} {} {}", c.red, c.green, c.blue);
  // let mut c = Color(255, 0, 0);

   //c.0 = 200;

  // println!("Color:{} {} {}", c.0, c.1, c.2);

  let mut p = Person::new("KARL", "STAHLMANN");
  println!("Person {} {}", p.f_name, p.l_name);
  println!("Person {}", p.full());
  p.set_last_name("STARKEMANN");
  println!("Person {}", p.full());
  println!("Person Tuple {:?}", p.to_tuple());



    
}