// constants
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    // mutability
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // shadowing
    let y = 5;

    let y = y + 1;

    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}");
    }

    println!("The value of y is: {y}");

    // tuples
    let tup: (isize, f64, char) = (500, 6.4, '😻');

    let (a, b, c) = tup;

    println!("The value of y is: {b}");

    println!("Tuple indexing: {}", c == tup.2);

    // arrays
    let arr = [3; 5]; // let arr = [3, 3, 3, 3, 3];

    println!("First element is: {}", arr[0]);
}
