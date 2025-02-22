use std::slice;

unsafe fn dangerous() {}

// calling C function
extern "C" {
    fn abs(input: i32) -> i32;
}

// let other languages to call Rust function
#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}

// static variables (global variables)
static HELLO_WORLD: &str = "Hello, world!";
static mut COUNTER: u32 = 0;

// unsafe traits
// rust can't guarantee that the trait is implemented safely even if it's MySync
unsafe trait MySync {}
// programmer must ensure that the trait is implemented safely for the type
unsafe impl MySync for i32 {}

fn main() {
    // raw pointer
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    // deref should be done in unsafe block
    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }

    let address = 0x012345usize;
    let r = address as *const i32;

    // unsafe function
    unsafe {
        dangerous();
    }

    // usually unsafe code is wrapped in safe function
    fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
        let len = values.len();
        let ptr = values.as_mut_ptr();

        assert!(mid <= len);

        unsafe {
            (
                slice::from_raw_parts_mut(ptr, mid),
                slice::from_raw_parts_mut(ptr.add(mid), len - mid),
            )
        }
    }

    let mut v = vec![1, 2, 3, 4, 5, 6];
    let (a, b) = split_at_mut(&mut v, 3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    // using extern functions
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }

    // accessing/modifying immutable static variables is safe
    println!("name is: {HELLO_WORLD}");

    // accessing/modifying mutable static variables is unsafe
    unsafe {
        COUNTER += 1;
        println!("COUNTER: {}", COUNTER);
    }
}
