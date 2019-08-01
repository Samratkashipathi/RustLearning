fn main() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result);
}


fn longest<'a, 'b>(x:&'a str, y:&'b str) -> &'a str{
    if x.len() > y.len(){
        &x
    }
    else{
        &y
    }
}