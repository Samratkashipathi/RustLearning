fn largest_number(vec: &Vec<i32>) -> i32 {
    let mut largest = vec[0];

    for &i in vec.iter() {
        if i > largest {
            largest = i;
        }
    }

    largest
}

fn generic_largest<T>(vec: &[T]) -> T 
    where T : PartialOrd + Copy
{
    let mut largest = vec[0];

    for &i in vec.iter() {
        if i > largest {
            largest = i;
        }
    }

    largest
}

fn main() {
    println!("Hello world!");

    let vec = vec![5, 6, 3, 2, 1];

    println!("Largest in vector {:#?} is {:#?}", vec, largest_number(&vec));

    println!("Largest in vector {:#?} is {:#?}", vec, generic_largest(&vec));

    let char_vec = ['z', 'a', 'b', 'c', 's'];

    println!("Largest in vector {:#?} is {:#?}", char_vec, generic_largest(&char_vec));
}