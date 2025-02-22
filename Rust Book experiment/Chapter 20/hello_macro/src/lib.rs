// declarative macro
#[macro_export]
macro_rules! my_vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

// procedural macro
// implementation in hello_macro_derive
pub trait HelloMacro {
    fn hello_macro();
}
