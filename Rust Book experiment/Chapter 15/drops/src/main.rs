struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer {
        data: String::from("C"),
    };
    let d = CustomSmartPointer {
        data: String::from("D"),
    };
    let e = CustomSmartPointer {
        data: String::from("E"),
    };
    println!("CustomSmartPointers created.");
    drop(d); // std::mem::drop, not d.drop()
    println!("CustomSmartPointer dropped before the end of main.");
}
