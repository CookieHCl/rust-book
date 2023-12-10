struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// field init shorthand
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

// tuple structs
struct Color(i32, i32, i32);
struct UnitLikeStruct;

fn main() {
    // mutating instance
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");

    // struct update
    let user2 = User {
        email: String::from("anothers@example.com"),
        ..user1
    };

    // tuple structs
    let black = Color(0, 0, 0);
    let unit_like = UnitLikeStruct;
}
