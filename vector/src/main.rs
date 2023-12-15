use std::slice::Iter;

fn main() {
    let mut v: Vec<i32> = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    for n_ref in &v {
        let n_plus_one = *n_ref + 1;
        println!("{n_plus_one}");
    }

    let w = vec![1, 2, 3, 4, 5];

    let third: &i32 = &w[2];
    println!("The third element is {third}");

    let third: Option<&i32> = w.get(2);
    match third {
        Some(t) => println!("The third element is {t}"),
        None => println!("There is no third element."),
    }

    let mut v = vec![100, 32, 57];
    for n_ref in &mut v {
        // n_ref has type &mut i32
        *n_ref += 50;
    }

    // iterators
    let mut v: Vec<i32> = vec![1, 2];
    let mut iter: Iter<'_, i32> = v.iter();
    let n1: &i32 = iter.next().unwrap();
    let n2: &i32 = iter.next().unwrap();
    let end: Option<&i32> = iter.next(); // None
}
