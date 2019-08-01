struct IDCard {
  name : String,
  age : i32
}

impl IDCard{
  fn create_new(name : String, age: i32) -> IDCard{
    IDCard {name:name, age:age}
  }
}

fn main(){
  println!("Hello world!");

  let id_card = IDCard::create_new(String::from("Samrat K S"), 22);

  println!("Name : {} -> Age : {}",id_card.name, id_card.age)
}