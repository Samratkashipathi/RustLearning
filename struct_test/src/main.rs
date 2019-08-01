#[derive(Debug)]
struct Rectangle{
  width: u32,
  height: u32,
}

fn main(){
    println!("Test Rust struct");
    let rect = Rectangle {width:10,height:10};
    println!("Rectangle dimensions:{:#?}",rect);
}
