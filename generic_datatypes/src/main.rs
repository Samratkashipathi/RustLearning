#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

#[derive(Debug)]
struct Square<T, U> {
    width: U,
    height: T,
    diagonals: T,
} // T is of type one U is another type

fn main() {
    let var = Point { x: 10, y: 20 };
    println!("Point is {:#?}", var);

    // Below is error
    // let var2 = Point { x: 10, y: 3.2 };
    // println!("Point2 is {:#?}", var2)

    let sq = Square { width: 3.2, height: 10, diagonals: 2 };
    println!("Square is {:#?}", sq);
}