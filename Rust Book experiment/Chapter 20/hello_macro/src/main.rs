use hello_macro::my_vec;
use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;

fn main() {
    let v: Vec<u32> = my_vec![1, 2, 3];
    println!("{:?}", v);

    Pancakes::hello_macro();
}
